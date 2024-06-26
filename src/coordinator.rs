use std::sync::atomic::AtomicU16;

use tokio::time::Instant;
use tracing::{debug, warn};

use crate::strategies::{self, FileStatus, ScanStrategyResult};

static COORDINATOR_ID: AtomicU16 = AtomicU16::new(0);

pub struct ScanCoordinator<'a> {
    id: u16,
    channel: crossbeam::channel::Sender<ScanStrategyResult>,
    pub paths: &'a [String],
    pub strategies: Vec<Box<dyn crate::strategies::ScanStrategy>>,
    pub update: bool,
}

impl<'a> ScanCoordinator<'a> {
    pub fn new(
        update: bool,
        paths: &'a [String],
        channel: crossbeam::channel::Sender<ScanStrategyResult>,
    ) -> ScanCoordinator {
        ScanCoordinator {
            id: COORDINATOR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            channel,
            paths,
            strategies: vec![Box::new(strategies::SHA256FileScanStrategy::new())],
            update,
        }
    }

    pub fn run(&self) {
        let start = Instant::now();
        debug!(
            workload = self.paths.len(),
            id = self.id,
            "running scan coordinator"
        );
        for path in self.paths {
            let path = std::path::Path::new(path);
            self.process_entry(path);
        }
        debug!(
            id = self.id,
            duration = ?start.elapsed(),
            "scan coordinator finished"
        );
    }

    pub fn process_entry(&self, path: &std::path::Path) {
        if path.is_dir() {
            return;
        }
        match std::fs::read(path) {
            Ok(data) => {
                for strategy in &self.strategies {
                    if self.update {
                        strategy.update(path, &data);
                    } else {
                        let result = strategy.process(path, &data);
                        let _ = self.channel.send(ScanStrategyResult {
                            strategy: strategy.get_name().to_string(),
                            result,
                        });
                    }
                }
            }
            Err(e) => {
                let _ = self.channel.send(ScanStrategyResult {
                    strategy: String::from("Coordinator"),
                    result: FileStatus::ReadFailed(path.to_str().unwrap().to_owned(), e),
                });
            }
        }
    }
}
