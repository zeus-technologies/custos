use std::sync::atomic::AtomicU16;

use tokio::time::Instant;
use tracing::{debug, warn};

use crate::strategies;

static COORDINATOR_ID: AtomicU16 = AtomicU16::new(0);

pub struct ScanCoordinator<'a> {
    id: u16,
    pub paths: &'a [String],
    pub strategies: Vec<Box<dyn crate::strategies::ScanStrategy>>,
}

impl<'a> ScanCoordinator<'a> {
    pub fn new(paths: &'a [String]) -> ScanCoordinator {
        ScanCoordinator {
            id: COORDINATOR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            paths,
            strategies: vec![Box::new(strategies::SHA256FileScanStrategy::new())],
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
                    strategy.process(path, &data);
                }
            }
            Err(e) => warn!("failed to read file: {:?}", e),
        }
    }
}
