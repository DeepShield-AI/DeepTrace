use aya::{
	Ebpf,
	maps::{AsyncPerfEventArray, HashMap},
	util::online_cpus,
};
use bytes::BytesMut;
use cache::Cache;
use clap::Parser;
use log::{debug, warn};
use mercury_common::structs::Data;
use process::{construct_spans, ebpf_output, spans_output};
use std::{ffi::CStr, sync::Arc};
use tokio::{
	fs::OpenOptions,
	io::{self, AsyncWrite, AsyncWriteExt, BufWriter},
	signal,
	sync::{Mutex, mpsc},
	task::JoinSet,
};

mod attach;
mod cache;
mod process;
mod span;
mod utils;

#[derive(Debug, Parser)]
struct Opts {
	#[clap(long, default_value = "tests/output/ebpf.txt")]
	file: String,
	#[clap(long, conflicts_with_all = ["file", "no_output"])]
	stdout: bool,
	#[clap(long, conflicts_with_all = ["file", "stdout"])]
	no_output: bool,
	#[arg(short, long, value_delimiter = ',', value_parser = clap::value_parser!(u32))]
	pids: Vec<u32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let opt = Opts::parse();
	env_logger::init();
	// Bump the memlock rlimit. This is needed for older kernels that don't use the
	// new memcg based accounting, see https://lwn.net/Articles/837122/
	let rlim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY };
	let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
	if ret != 0 {
		debug!("remove limit on locked memory failed, ret is: {}", ret);
	}

	// This will include eBPF object file as raw bytes at compile-time and load it at runtime.
	// This approach is recommended for most real-world use cases. If you would like to specify the
	// eBPF program at runtime rather than at compile-time, you can reach for `Bpf::load_file` instead.
	let mut ebpf = Ebpf::load(aya::include_bytes_aligned!(concat!(env!("OUT_DIR"), "/mercury")))?;

	if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
		// This can happen if you remove all log statements from your eBPF program.
		warn!("failed to initialize eBPF logger: {}", e);
	}

	let pids = if opt.pids.is_empty() {
		utils::get_pids().await.expect("Get pids error.")
	} else {
		opt.pids
	};
	println!("Pids: {:?}", pids);
	let mut pids_map: HashMap<&mut aya::maps::MapData, u32, u32> =
		HashMap::try_from(ebpf.map_mut("pids").expect("Failure to take pids map."))?;

	for pid in pids {
		pids_map.insert(pid, 0, 0)?
	}

	attach::attach_tracepoint(&mut ebpf)?;
	let (ebpf_sender, ebpf_receiver) = mpsc::unbounded_channel();
	let (message_sender, message_receiver) = mpsc::unbounded_channel();
	let (span_sender, span_receiver) = mpsc::unbounded_channel();

	let cache = Cache::new();
	// Retrieve the perf event array from the eBPF program to read events from it.
	let mut perf_array = AsyncPerfEventArray::try_from(ebpf.take_map("events").unwrap())?;

	// Calculate the size of the Data structure in bytes.
	let len_of_data = size_of::<Data>();

	// Iterate over each online CPU core. For eBPF applications, processing is often done per CPU core.
	for cpu_id in online_cpus().expect("Get CPU id error") {
		// open a separate perf buffer for each cpu
		let mut buf = perf_array.open(cpu_id, Some(128))?;
		let tx = ebpf_sender.clone();
		let message = message_sender.clone();
		// process each perf buffer in a separate task
		tokio::spawn(async move {
			// Prepare a set of buffers to store the data read from the perf buffer.
			// Here, 16 buffers are created, each with a capacity equal to the size of the Data structure.
			let mut buffers =
				(0..16).map(|_| BytesMut::with_capacity(len_of_data)).collect::<Vec<_>>();
			loop {
				// Attempt to read events from the perf buffer into the prepared buffers.
				let events = match buf.read_events(&mut buffers).await {
					Ok(events) => events,
					Err(e) => {
						warn!("Error reading events: {}", e);
						continue;
					},
				};

				// Iterate over the number of events read. `events.read` indicates how many events were read.
				for i in 0..events.read {
					let buf = &mut buffers[i];
					let data = unsafe { *(buf.as_ptr() as *const Data) }; // Convert the buffer to a Data structure.
					message.send(data).expect("Error sending data");
					// handle_data(data, &tx).await.expect("error");
					let message = format!(
						"{}, {}, {}, {}, {}, {}, {}, length: {}, {}, {}, {}, {:?}\n",
						data.tgid,
						data.syscall,
						CStr::from_bytes_until_nul(&data.comm)
							.expect("command error")
							.to_string_lossy()
							.into_owned(),
						data.quintuple,
						data.timestamp_ns,
						data.enter_seq,
						data.exit_seq,
						data.payload.len,
						data.protocol,
						data.type_,
						data.uuid,
						data.buffer()
					);
					tx.send(message).expect("Sending message error");
				}
			}
		});
	}

	let ebpf_writer: Box<dyn AsyncWrite + Unpin + Send> = match (opt.stdout, opt.no_output) {
		(true, false) => Box::new(BufWriter::new(io::stdout())),
		(_, true) => Box::new(io::sink()),
		_ => {
			let file = OpenOptions::new()
				.create(true)
				.write(true)
				.truncate(true)
				.open(&opt.file)
				.await
				.expect("Failed to open file");
			Box::new(BufWriter::with_capacity(1024 * 1024, file))
		},
	};

	let span_writer = {
		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open("tests/output/spans.json")
			.await
			.expect("Failed to open file");
		Arc::new(Mutex::new(BufWriter::with_capacity(1024 * 1024, file)))
	};

	let writer = span_writer.clone();

	let mut tasks = JoinSet::new();

	tasks.spawn(ebpf_output(ebpf_writer, ebpf_receiver));
	tasks.spawn(construct_spans(cache, message_receiver, span_sender));
	tasks.spawn(spans_output(writer, span_receiver));

	let ctrl_c = signal::ctrl_c();
	println!("Waiting for Ctrl-C...");
	ctrl_c.await?;
	tasks.shutdown().await;

	let mut guard = span_writer.lock().await;
	guard.write_all(b"\t]\n}").await?;
	guard.flush().await?;
	println!("Exiting...");

	Ok(())
}
