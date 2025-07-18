// metric/module.rs
use crate::Module;
use crate::metric::cpu_usage::collector::CpuUsageCollector;
use crate::metric::cpu_usage::logger::CpuUsageLogger;
use crate::AgentError;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use std::io::Result as IoResult;
use log::{info, warn};

pub struct MetricModule {
    cpu_collector_handle: Option<thread::JoinHandle<()>>,
    cpu_collector_running: Arc<AtomicBool>,
     logger: Option<Arc<Mutex<CpuUsageLogger>>>,
}

impl MetricModule {
    pub fn new() -> IoResult<Self> {
    let logger = match CpuUsageLogger::new("cpu_usage.csv") {
        Ok(logger) => Some(Arc::new(Mutex::new(logger))),
        Err(e) => {
            warn!("Failed to create CPU usage logger: {}", e);
            None
        }
    };

    Ok(Self {
        cpu_collector_handle: None,
        cpu_collector_running: Arc::new(AtomicBool::new(false)),
        logger,
    })
}

    fn start_cpu_collector(&mut self) {
    let running = Arc::clone(&self.cpu_collector_running);

    let logger = match self.logger.as_ref() {
        Some(logger) => Arc::clone(logger),
        None => {
            warn!("Logger not initialized");
            return;
        }
    };

    self.cpu_collector_handle = Some(thread::spawn(move || {
        let collector = CpuUsageCollector::new(0);
        info!("CPU usage collector started");

        while running.load(Ordering::Relaxed) {
            let usages = collector.collect();
            for usage in &usages {
                info!("CPU {}: {:.2}%", usage.cpu_id, usage.usage);
                let mut logger = logger.lock().unwrap();
                logger.write(usage);
            }
            collector.sleep_duration();
        }

        let mut logger = logger.lock().unwrap();
        logger.flush();
        info!("CPU usage collector stopped");
    }));
}
}

impl Module for MetricModule {
    type Error = AgentError;

    fn name(&self) -> &str {
        "MetricCollector"
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        info!("Starting {}", self.name());

        self.cpu_collector_running.store(true, Ordering::Relaxed);
        self.start_cpu_collector();

        info!("{} started", self.name());
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Self::Error> {
        info!("Stopping {}", self.name());

        self.cpu_collector_running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.cpu_collector_handle.take() {
            if !handle.is_finished() {
                info!("Waiting for CPU collector to finish...");
                handle.join().expect("Failed to stop CPU collector");
            }
        }

        info!("{} stopped", self.name());
        Ok(())
    }
}