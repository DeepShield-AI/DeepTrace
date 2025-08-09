use super::{CpuCollectorManager, MetricError, DiskCollectorManager, MemCollectorManager,NetCollectorManager};
use crate::{app::runtime::spawn, metric::mem, Module};
use std::sync::{Arc, Mutex}; 
use crate::metric::{CpuLogger, DiskLogger, MemLogger,NetLogger}; 
use std::sync::atomic::{AtomicBool, Ordering};
use log::{info, warn};
use std::fs;
use aya::Ebpf; 
pub struct MetricCollector {
    cpu_collector: Option<CpuCollectorManager>,
    disk_collector: Option<DiskCollectorManager>,
    mem_collector: Option<MemCollectorManager>,
    net_collector: Option<NetCollectorManager>, // Add NetCollectorManager
}
impl MetricCollector {
      pub fn new(ebpf: Option<Arc<Mutex<Ebpf>>>) -> Result<Self, MetricError> {
        fs::create_dir_all("output")?;
        let disk_logger = Some(Arc::new(Mutex::new(DiskLogger::new("output/disk")?)));
        let disk_collector = match &ebpf {
            Some( ebpf_ref) => {
                Some(DiskCollectorManager::new(disk_logger).with_ebpf(ebpf_ref.clone()))
            },
            None => {
                Some(DiskCollectorManager::new(disk_logger))
            }
        };
        // 创建 CPU 收集器，如果有 ebpf 实例则使用它
        let cpu_collector = match &ebpf {
            Some( ebpf_ref) => {
                Some(CpuCollectorManager::new(
                    Some(Arc::new(Mutex::new(CpuLogger::new("output/cpu_usage.csv")?)))
                ).with_ebpf(ebpf_ref.clone()))
            },
            None => {
                Some(CpuCollectorManager::new(
                    Some(Arc::new(Mutex::new(CpuLogger::new("output/cpu_usage.csv")?)))
                ))
            }
        };
        
        
        
        let mem_logger = Some(Arc::new(Mutex::new(MemLogger::new("output/mem_usage.csv")?)));
        let mem_collector = match &ebpf {
            Some( ebpf_ref) => {
                Some(MemCollectorManager::new(mem_logger).with_ebpf(ebpf_ref.clone()))
            },
            None => {
                Some(MemCollectorManager::new(mem_logger))
            }
        };
        
        let net_logger = Some(Arc::new(Mutex::new(NetLogger::new("output/net_usage.csv")?)));
        let net_collector = Some(NetCollectorManager::new(net_logger));
        
        Ok(Self {
            cpu_collector,
            disk_collector,
            mem_collector,
            net_collector,
        })
    }
}

impl Module for MetricCollector {
    type Error = MetricError; // 定义 Error 类型

    fn name(&self) -> &str { // 实现 name 方法
        "Metric Collector"
    }
    fn start(&mut self) -> Result<(), Self::Error> {
        info!("Starting Metric Collector");

    
        if let Some(ref mut manager) = self.cpu_collector {
            manager.start_collector();
        } else {
            warn!("CPU collector not initialized");
        }


        if let Some(ref mut manager) = self.disk_collector {
            manager.start_collector();
        } else {
            warn!("Disk collector not initialized");
        }

       
        if let Some(ref mut manager) = self.mem_collector {
            manager.start_collector();
        } else {
            warn!("Mem collector not initialized");
        }

       
        if let Some(ref mut manager) = self.net_collector {
            manager.start_collector();
        } else {
            warn!("Net collector not initialized");
        }
        info!("Metric Collector started");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Self::Error> {
        info!("Stopping Metric Collector");

        
        if let Some(ref mut manager) = self.cpu_collector {
            manager.stop_collector().await;
        }

    
        if let Some(ref mut manager) = self.disk_collector {
            manager.stop_collector().await;
        }


        if let Some(ref mut manager) = self.mem_collector {
            manager.stop_collector().await;
        }

        
        if let Some(ref mut manager) = self.net_collector {
            manager.stop_collector().await;
        }

        info!("Metric Collector stopped");
        Ok(())
    }
}