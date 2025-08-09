use super::{TraceError, attach, loader, utils};
use std::fs::OpenOptions;
use std::io::Write;
use crate::{
	Module,
	app::runtime::{block_on, spawn_blocking},
	config::{TraceAccess, trace_config},
	utils::sys,

};
use trace_common::{
    protocols::l7::L7Protocol,
    structs::{Syscall,Data},
};
use arc_swap::access::Access;
use aya::{Ebpf, maps::AsyncPerfEventArray, util::online_cpus};
use bytes::BytesMut;
use crossbeam_channel::Sender;
use log::{info, warn};
use std::time::Duration;
use tokio::{
	task::JoinHandle,
	time::{self, timeout},
};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
pub struct TraceModule {
	config: TraceAccess,
	handles: Option<Vec<JoinHandle<()>>>,
	output: Sender<Data>,
	pub ebpf: Arc<Mutex<Ebpf>>,
	running: Arc<AtomicBool>,
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
		Ok(Self { config, handles: None, output, ebpf: Arc::new(Mutex::new(ebpf)) , running: Arc::new(AtomicBool::new(false)),})
	}
	pub fn shared_ebpf(&self) -> Arc<Mutex<Ebpf>> {
        self.ebpf.clone()
    }
}

impl Module for TraceModule {
	type Error = TraceError;
	fn name(&self) -> &str {
		"Trace"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
        if self.running.swap(true, Ordering::Relaxed) {
            return Ok(());
        }
        
        info!("Starting {} module...", self.name());
        let config = self.config.load();
        
        // 获取 ebpf 锁
        let mut ebpf_guard = self.ebpf.lock().unwrap();
        utils::config_pids(&mut ebpf_guard, config.pids.clone())?;
        attach::attach_tracepoint(&mut ebpf_guard).expect("Failed to attach tracepoint");
        
        // Retrieve the perf event array from the eBPF program to read events from it.
        let mut perf_array = AsyncPerfEventArray::try_from(ebpf_guard.take_map("events").unwrap())
            .expect("Failed to take perf array");
            
        // release ebpf guard
        drop(ebpf_guard);
        
        // Calculate the size of the Data structure in bytes.
        let len_of_data = size_of::<Data>();
        let mut handlers = vec![];
        let running = self.running.clone();

        // Iterate over each online CPU core.
        for cpu_id in online_cpus().expect("Get CPU id error") {
            // open a separate perf buffer for each cpu
            let mut buf = perf_array.open(cpu_id, Some(128)).expect("Failed to open perf buffer");
            let output = self.output.clone();
            let running = running.clone();
            
            let handle = spawn_blocking(move || {
                block_on(async {
                    let mut buffers =
                        (0..16).map(|_| BytesMut::with_capacity(len_of_data)).collect::<Vec<_>>();

                    let timeout = Duration::from_millis(100);
                    while running.load(Ordering::Relaxed){
                        let events =
                            match time::timeout(timeout, buf.read_events(&mut buffers)).await {
                                Ok(events) => events.unwrap(),
                                Err(_e) => {
                                    continue;
                                },
                            };
                        for buf in buffers.iter_mut().take(events.read) {
                            let data = unsafe { *(buf.as_ptr() as *const Data) };
                            output.send(data).expect("Error sending data");
                        }
                    }
                })
            });
            handlers.push(handle);
        }
        self.handles = Some(handlers);
        Ok(())
    }
	async fn stop(&mut self) -> Result<(), Self::Error> {
		println!("stop before threads collcected");
		if !self.running.swap(false, Ordering::SeqCst) {
        return Ok(());
    }
		if let Some(handles) = self.handles.take() {
		
			for handle in handles {
			
				if !handle.is_finished() {
					
					info!("Waiting for {} module to stop...", self.name());
					handle.await.expect("Failed to stop trace module");
				}
			}	
		}
	
		println!("{} module stopped.", self.name());
			Ok(())
		}
}
