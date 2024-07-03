use tracing::debug;

use crate::strategies;

use super::Reporter;

pub struct ReportManager {
    channel: crossbeam::channel::Receiver<strategies::ScanStrategyResult>,
    reporters: Vec<Box<dyn Reporter>>,
}

impl ReportManager {
    pub fn new(
        channel: crossbeam::channel::Receiver<strategies::ScanStrategyResult>,
    ) -> ReportManager {
        ReportManager {
            channel,
            reporters: Vec::new(),
        }
    }

    pub fn add_reporter(&mut self, reporter: Box<dyn Reporter>) {
        self.reporters.push(reporter);
    }

    pub fn process_results(&self) {
        loop {
            match self.channel.recv() {
                Ok(r) => {
                    for reporter in &self.reporters {
                        reporter.report(&r);
                    }
                }
                Err(e) => {
                    debug!("closing: {:?}", e);
                    break;
                }
            };
        }
    }
}
