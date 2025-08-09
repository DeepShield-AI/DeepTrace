// mem/start.rs

use crate::metric::{MemCollector, MemLogger};
use crate::{Module, app::runtime::spawn};
use crate::app::runtime::{spawn_blocking, block_on};
use std::{
    sync::{
        Arc,
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::task::JoinHandle;
use log::{info, warn};
use aya::Ebpf;
pub struct MemCollectorManager {
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    logger: Option<Arc<Mutex<MemLogger>>>,
    ebpf: Option<Arc<Mutex<Ebpf>>>,
}

impl MemCollectorManager {
    pub fn new(logger: Option<Arc<Mutex<MemLogger>>>) -> Self {
        Self {
            handle: None,
            running: Arc::new(AtomicBool::new(false)),
            logger,
            ebpf: None,
        }
    }
    pub fn with_ebpf(mut self, ebpf: Arc<Mutex<Ebpf>>) -> Self {
        self.ebpf = Some(ebpf);
        self
    }
    pub fn start_collector(&mut self) {
        if self.running.swap(true, Ordering::Relaxed) {
            return;
        }

        let running = Arc::clone(&self.running);
        let logger = match self.logger.as_ref() {
            Some(logger) => Arc::clone(logger),
            None => {
                warn!("Logger not initialized");
                return;
            }
        };
        
        let ebpf = self.ebpf.clone();

        self.handle = Some(spawn_blocking(move || {
            block_on(async move {
                let collector = MemCollector::new();
                while running.load(Ordering::Relaxed) {
                    let metrics = if let Some(ebpf_ref) = &ebpf {
                        let ebpf_lock = ebpf_ref.lock().unwrap();
                        collector.collect(Some(&*ebpf_lock));
                    } else {
                        collector.collect(None)
                    };
                    
                    if let Ok(mut logger) = logger.lock() {
                        for metric in metrics {
                            logger.write(&metric);
                        }
                    }
                    collector.sleep_duration().await;
                }

                if let Ok(mut logger) = logger.lock() {
                    logger.flush();
                }
                info!("Memory usage collector stopped");
            })
        }));
    }
    pub async fn stop_collector(&mut self) {
        if !self.running.swap(false, Ordering::Relaxed) {
            warn!("Memory collector is not running");
            return;
        }

        if let Some(handle) = self.handle.take() {
            if !handle.is_finished() {
                info!("Waiting for Memory collector to finish...");
                handle.await.expect("Failed to stop Memory collector");
            }
        }
    }
}