use aya::{
	Ebpf,
	maps::{HashMap, RingBuf, AsyncPerfEventArray},
	util::online_cpus,
};
use log::error;
#[rustfmt::skip]
use log::{debug, warn};
use mercury_common::Data;
use process::handle_data;
use tokio::{
	fs::{self, OpenOptions},
	io::{AsyncWriteExt, BufWriter},
	signal,
	sync::mpsc,
};
use bytes::BytesMut;

const CHANNEL_CAPACITY: usize = 65536;
const FLUSH_INTERVAL: usize = 1000;

mod attach;
mod process;
mod utils;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

	let pids = utils::get_pids().await.expect("Get pids error.");

	let mut pids_map: HashMap<&mut aya::maps::MapData, u32, u32> =
		HashMap::try_from(ebpf.map_mut("pids").expect("Failure to take pids map."))?;

	for pid in pids {
		pids_map.insert(pid, 0, 0)?
	}

	attach::attach_ingress(&mut ebpf)?;
	attach::attach_egress(&mut ebpf)?;
	fs::create_dir_all("experiments").await?;
	// Retrieve the perf event array from the eBPF program to read events from it.
	let mut perf_array = AsyncPerfEventArray::try_from(ebpf.take_map("events").unwrap())?;
	let (global_tx, mut global_rx) = mpsc::channel::<String>(CHANNEL_CAPACITY);
	// Calculate the size of the Data structure in bytes.
	let len_of_data = 16384u32;
	// Iterate over each online CPU core. For eBPF applications, processing is often done per CPU core.
	for cpu_id in online_cpus().expect("error") {
		// open a separate perf buffer for each cpu
		let mut buf = perf_array.open(cpu_id, Some(64))?;
		let tx = global_tx.clone();
		// process each perf buffer in a separate task
		tokio::spawn(async move {
			// Prepare a set of buffers to store the data read from the perf buffer.
			// Here, 10 buffers are created, each with a capacity equal to the size of the Data structure.
			let mut buffers = (0..10)
				.map(|_| BytesMut::with_capacity(len_of_data as usize))
				.collect::<Vec<_>>();
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
					handle_data(data, &tx).await.expect("error");
				}
			}
		});
	}

    tokio::spawn(async move {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("experiments/output.txt")
            .await
            .expect("Failed to open file");
        
        let mut writer = BufWriter::with_capacity(1024 * 1024, file);
        let mut count = 0;

        while let Some(line) = global_rx.recv().await {
            if let Err(e) = writer.write_all(line.as_bytes()).await {
                error!("Failed to write to file: {}", e);
                break;
            }

            count += 1;
            if count % FLUSH_INTERVAL == 0 {
                if let Err(e) = writer.flush().await {
                    error!("Failed to flush buffer: {}", e);
                    break;
                }
            }
        }

        if let Err(e) = writer.flush().await {
            error!("Final flush failed: {}", e);
        }
    });

	let ctrl_c = signal::ctrl_c();
	println!("Waiting for Ctrl-C...");
	ctrl_c.await?;
	println!("Exiting...");

	Ok(())
}
