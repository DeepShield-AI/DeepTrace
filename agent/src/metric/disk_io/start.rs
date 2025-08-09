use crate::metric::{DiskCollector, DiskLogger};
use crate::{Module, app::runtime::spawn};
use crate::app::runtime::{spawn_blocking, block_on};
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    time::Duration,
    collections::HashMap,
};
use tokio::task::JoinHandle;
use log::{info, warn};
use aya::Ebpf;
pub struct DiskCollectorManager {
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    logger: Option<Arc<Mutex<DiskLogger>>>,
    ebpf:Option<Arc<Mutex<Ebpf>>>,
}

impl DiskCollectorManager {
    pub fn new(logger: Option<Arc<Mutex<DiskLogger>>>) -> Self {
        Self {
            handle: None,
            running: Arc::new(AtomicBool::new(false)),
            logger,
            ebpf:None,
        }
    }
    pub fn with_ebpf(mut self, ebpf: Arc<Mutex<Ebpf>>)->Self{
       self.ebpf=Some(ebpf);
       self
   }
    pub fn start_collector(&mut self) {
        if self.running.swap(true, Ordering::Relaxed) {
            return;
        }
        let start_time = SystemTime::now();
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
                let mut collector = DiskCollector::new();
                while running.load(Ordering::Relaxed) {
                    // 收集指标
                    let mut metrics = collector.collect_metrics();
                    
                     let ebpf_metrics = if let Some(ebpf_ref) = &ebpf {
                        let ebpf_lock = ebpf_ref.lock().unwrap();
                        collector.collect_ebpf_metrics(Some(&*ebpf_lock))
                    } else {
                        collector.collect_ebpf_metrics(None)
                    };
                    
                    let usages = collector.collect_usages();
                    let ext4_cache_stats = collector.collect_ext4_cache_stats();
                    
                    // 写入日志
                    let mut logger = logger.lock().unwrap();
                    logger.write_metrics(&metrics);
                    logger.write_usages(&usages);
                   
                    logger.write_ebpf_metrics(&ebpf_metrics); 
                    match ext4_cache_stats {
                        Ok(stats) => logger.write_ext4_cache(&stats),
                        Err(e) => warn!("Failed to collect ext4 cache stats: {}", e),
                    }
                    
                    // 等待下一次采集
                    collector.sleep_duration().await;
                }
                
                // 刷新日志
                let mut logger = logger.lock().unwrap();
                logger.flush();
                info!("Disk metrics collector stopped");
            });
        }));
    }

    pub async fn stop_collector(&mut self) {
        if !self.running.swap(false, Ordering::Relaxed) {
            warn!("Disk collector is not running");
            return;
        }
        
        if let Some(handle) = self.handle.take() {
            if !handle.is_finished() {
                info!("Waiting for disk collector to finish...");
                handle.await.expect("Failed to stop disk collector");
            }
        }
    }
}