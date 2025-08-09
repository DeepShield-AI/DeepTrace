// cpu/start.rs
use crate::metric::{CpuCollector, CpuLogger};
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

pub struct CpuCollectorManager {
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    logger: Option<Arc<Mutex<CpuLogger>>>,
    ebpf:Option<Arc<Mutex<Ebpf>>>,
}

impl CpuCollectorManager {
    pub fn new(logger: Option<Arc<Mutex<CpuLogger>>>) -> Self {
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
        return ;
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
        let collector = CpuCollector::new();
        while running.load(Ordering::Relaxed) {
           
            let usages = if let Some(ebpf_ref) = &ebpf {
                        // 传递 ebpf 实例给 collect 函数
                        let ebpf_lock = ebpf_ref.lock().unwrap();
               
                        collector.collect(Some(&*ebpf_lock))
                    } else {
                        // 不传递 ebpf 实例
               
                        collector.collect(None)
                    };
            for usage in &usages {
          
                let logger = logger.lock().unwrap();
                logger.write(usage); // 同步 I/O，现在在阻塞线程中执行
            }
            collector.sleep_duration().await;
        }

        let logger = logger.lock().unwrap();
        logger.flush();
        info!("CPU usage collector stopped");
    })
}));
}

    pub async fn stop_collector(&mut self) {
        if !self.running.swap(false, Ordering::Relaxed) {
            warn!("CPU collector is not running");
            return;
        }

        if let Some(handle) = self.handle.take() {
            if !handle.is_finished() {
                info!("Waiting for CPU collector to finish...");
                handle.await.expect("Failed to stop CPU collector");
            }
        }
    }
}