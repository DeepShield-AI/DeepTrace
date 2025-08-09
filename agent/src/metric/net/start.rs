// net/start.rs
use crate::metric::{NetCollector, NetLogger};
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

pub struct NetCollectorManager {
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    logger: Option<Arc<Mutex<NetLogger>>>,
}

impl NetCollectorManager {
    pub fn new(logger: Option<Arc<Mutex<NetLogger>>>) -> Self {
        Self {
            handle: None,
            running: Arc::new(AtomicBool::new(false)),
            logger,
        }
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

        self.handle = Some(spawn_blocking(move || {
            block_on(async move {
                let collector = NetCollector::new();
                while running.load(Ordering::Relaxed) {
                    let metrics = collector.collect();
                    if let Ok(mut logger) = logger.lock() {
                        logger.write(&metrics);
                    }
                    collector.sleep_duration().await;
                }

                if let Ok(mut logger) = logger.lock() {
                    logger.flush();
                }
                info!("Network usage collector stopped");
            })
        }));
    }

    pub async fn stop_collector(&mut self) {
        if !self.running.swap(false, Ordering::Relaxed) {
            warn!("Network collector is not running");
            return;
        }

        if let Some(handle) = self.handle.take() {
            if !handle.is_finished() {
                info!("Waiting for Network collector to finish...");
                handle.await.expect("Failed to stop Network collector");
            }
        }
    }
}