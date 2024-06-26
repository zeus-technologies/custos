use tracing::{debug, trace, warn};

use crate::strategies::{self, FileStatus};

pub struct Reporter {
    channel: crossbeam::channel::Receiver<strategies::ScanStrategyResult>,
}

impl Reporter {
    pub fn new() -> (
        Reporter,
        crossbeam::channel::Sender<strategies::ScanStrategyResult>,
    ) {
        let (s, r) = crossbeam::channel::unbounded();
        (Reporter { channel: r }, s)
    }

    pub fn process_results(&self) {
        loop {
            match self.channel.recv() {
                Ok(r) => match r.result {
                    FileStatus::OK(path) => trace!("file passed: {:?}", path),
                    FileStatus::NewFile(path) => {
                        trace!("{:?}: found new file: {:?}", r.strategy, path)
                    }
                    FileStatus::FileChanged(path) => {
                        trace!("{:?}: file was changed: {:?}", r.strategy, path)
                    }
                    FileStatus::MaliciousFile(path) => {
                        trace!("{:?}: malicious file found: {:?}", r.strategy, path)
                    }
                    FileStatus::ReadFailed(path, error) => {
                        trace!("{:?}: failed to read file: {:?}", path, error)
                    }
                },
                Err(e) => {
                    debug!("closing: {:?}", e);
                    break;
                }
            }
        }
    }
}
