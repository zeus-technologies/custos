use std::sync::atomic::AtomicU16;

use tokio::time::Instant;
use tracing::debug;

use crate::strategies::{FileStatus, StrategyResult};

static COORDINATOR_ID: AtomicU16 = AtomicU16::new(0);

pub struct ScanCoordinator<'a> {
    id: u16,
    channel: crossbeam::channel::Sender<StrategyResult>,
    pub paths: &'a [String],
    scan_strategies: Vec<Box<dyn crate::strategies::ScanStrategy>>,
    process_strategies: Vec<Box<dyn crate::strategies::ProcessStrategy>>,
    pub update: bool,
}

impl<'a> ScanCoordinator<'a> {
    pub fn new(
        update: bool,
        paths: &'a [String],
        channel: crossbeam::channel::Sender<StrategyResult>,
    ) -> ScanCoordinator {
        ScanCoordinator {
            id: COORDINATOR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            channel,
            paths,
            scan_strategies: Vec::new(),
            process_strategies: Vec::new(),
            update,
        }
    }

    pub fn add_scan_strategy(&mut self, strategy: Box<dyn crate::strategies::ScanStrategy>) {
        self.scan_strategies.push(strategy);
    }

    pub fn add_process_strategy(&mut self, strategy: Box<dyn crate::strategies::ProcessStrategy>) {
        self.process_strategies.push(strategy);
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
                for strategy in &self.scan_strategies {
                    if self.update {
                        strategy.update(path, &data);
                    } else {
                        let result = strategy.process(path, &data);
                        for process_strategy in &self.process_strategies {
                            if let Some(process_result) =
                                process_strategy.process(&result, path, &data)
                            {
                                let _ = self.channel.send(StrategyResult {
                                    strategy: process_strategy.get_name().to_string(),
                                    result: process_result,
                                });
                            }
                        }
                        let _ = self.channel.send(StrategyResult {
                            strategy: strategy.get_name().to_string(),
                            result,
                        });
                    }
                }
            }
            Err(e) => {
                let _ = self.channel.send(StrategyResult {
                    strategy: String::from("Coordinator"),
                    result: FileStatus::ReadFailed(path.to_str().unwrap().to_owned(), e),
                });
            }
        }
    }
}
