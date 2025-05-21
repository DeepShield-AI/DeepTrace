use super::{TraceError, attach, loader, utils};
use crate::{
	AgentError, Module,
	app::runtime::spawn,
	config::{TraceAccess, trace_config},
	utils::sys,
};
use arc_swap::{Cache, access::Access};
use aya::{Ebpf, maps::AsyncPerfEventArray, util::online_cpus};
use bytes::BytesMut;
use crossbeam_channel::Sender;
use log::{info, warn};
use tokio::task::JoinHandle;
use trace_common::structs::Data;

pub struct TraceModule {
	config: TraceAccess,
	handles: Option<Vec<JoinHandle<()>>>,
	output: Sender<Data>,
	ebpf: Ebpf,
}

impl TraceModule {
	pub fn new(output: Sender<Data>) -> Result<Self, TraceError> {
		let config = trace_config();
		sys::unlock_memory();
		let mut ebpf = loader::load_trace()?;
		if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
			// This can happen if you remove all log statements from your eBPF program.
			warn!("Failed to initialize eBPF logger: {}", e);
		}
		Ok(Self { config, handles: None, output, ebpf })
	}
}

impl Module for TraceModule {
	type Error = TraceError;
	fn name(&self) -> &str {
		"Trace"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
		info!("Starting {} module...", self.name());
		let config = self.config.load();
		utils::config_pids(&mut self.ebpf, config.pids.clone())?;
		attach::attach_tracepoint(&mut self.ebpf).expect("Failed to attach tracepoint");

		// Retrieve the perf event array from the eBPF program to read events from it.
		let mut perf_array = AsyncPerfEventArray::try_from(self.ebpf.take_map("events").unwrap())
			.expect("Failed to take perf array");

		// Calculate the size of the Data structure in bytes.
		let len_of_data = size_of::<Data>();
		let mut handlers = vec![];
		// Iterate over each online CPU core. For eBPF applications, processing is often done per CPU core.
		for cpu_id in online_cpus().expect("Get CPU id error") {
			// open a separate perf buffer for each cpu
			let mut buf = perf_array.open(cpu_id, Some(128)).expect("Failed to open perf buffer");
			let output = self.output.clone();
			// process each perf buffer in a separate task
			let handle = spawn(async move {
				// Prepare a set of buffers to store the data read from the perf buffer.
				// Here, 16 buffers are created, each with a capacity equal to the size of the Data structure.
				let mut buffers =
					(0..16).map(|_| BytesMut::with_capacity(len_of_data)).collect::<Vec<_>>();
				loop {
					// Attempt to read events from the perf buffer into the prepared buffers.
					let events = match buf.read_events(&mut buffers).await {
						Ok(events) => events,
						Err(e) => {
							warn!("Error reading events: {e}");
							continue;
						},
					};

					// Iterate over the number of events read. `events.read` indicates how many events were read.
					for i in 0..events.read {
						let buf = &mut buffers[i];
						let data = unsafe { *(buf.as_ptr() as *const Data) }; // Convert the buffer to a Data structure.
						output.send(data).expect("Error sending data");
					}
				}
			});
			handlers.push(handle);
		}
		self.handles = Some(handlers);
		Ok(())
	}
	fn stop(&mut self) -> Result<(), Self::Error> {
		if let Some(handles) = self.handles.take() {
			for handle in handles {
				if !handle.is_finished() {
					info!("Waiting for {} module to stop...", self.name());
					handle.abort();
				}
			}
		}
		println!("{} module stopped.", self.name());
		Ok(())
	}
}
