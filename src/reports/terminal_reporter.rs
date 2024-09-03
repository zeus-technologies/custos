use tracing::trace;

use crate::strategies::FileStatus;

pub struct TerminalReporter {
    quiet: bool,
}

impl TerminalReporter {
    pub fn new(quiet: bool) -> TerminalReporter {
        TerminalReporter { quiet }
    }
}

impl super::Reporter for TerminalReporter {
    fn report(&self, result: &crate::strategies::StrategyResult) {
        match &result.result {
            FileStatus::OK(path) => trace!("file passed: {:?}", path),
            FileStatus::NewFile(path) => {
                println!("{}: found new file: {}", result.strategy, path)
            }
            FileStatus::FileChanged(path) => {
                println!("{}: file was changed: {}", result.strategy, path)
            }
            FileStatus::MaliciousFile(path) => {
                println!("{}: malicious file found: {}", result.strategy, path)
            }
            FileStatus::ReadFailed(path, error) => {
                if !self.quiet {
                    println!("{}: failed to read file: {}", path, error)
                }
            }
        }
    }
}
