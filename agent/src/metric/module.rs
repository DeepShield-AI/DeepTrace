// module.rs
use super::{CpuCollectorManager, MetricError};
use crate::{Module, app::runtime::spawn};
use std::sync::{Arc, Mutex}; 
use crate::metric::CpuLogger; 
use std::sync::atomic::{AtomicBool, Ordering};
use log::{info, warn};

pub struct MetricCollector {
    cpu_collector: Option<CpuCollectorManager>,
}

impl MetricCollector {
    pub fn new() -> Result<Self, MetricError> {
        let cpu_collector: Option<CpuCollectorManager> = match CpuCollectorManager::new(Some(Arc::new(Mutex::new(CpuLogger::new("cpu_usage.csv")?)))) {
            manager => Some(manager),
        };

        Ok(Self {
            cpu_collector,
        })
    }

    fn start_cpu_collector(&mut self) {
        if let Some(ref mut manager) = self.cpu_collector {
            manager.start_collector();
        } else {
            warn!("CPU collector not initialized");
        }
    }
}

impl Module for MetricCollector {
    type Error = MetricError;

    fn name(&self) -> &str {
        "Metric Collector"
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        println!("Starting {}", self.name());

        if self.cpu_collector.is_none() {
            warn!("CPU collector is not initialized");
            return Ok(());
        }

        self.start_cpu_collector();

        info!("{} started", self.name());
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Self::Error> {
        info!("Stopping {}", self.name());

        if let Some(ref mut manager) = self.cpu_collector {
            manager.stop_collector().await;
        } else {
            warn!("CPU collector is not running");
        }

        println!("{} stopped", self.name());
        Ok(())
    }
}