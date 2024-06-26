mod hash_strategy;

use std::io::Error;

pub use hash_strategy::SHA256FileScanStrategy;

pub trait ScanStrategy {
    fn get_name(&self) -> &str;
    fn process(&self, path: &std::path::Path, data: &[u8]) -> FileStatus;
    fn update(&self, path: &std::path::Path, data: &[u8]);
}

#[derive(Debug)]
pub enum FileStatus {
    NewFile(String),
    FileChanged(String),
    MaliciousFile(String),
    ReadFailed(String, Error),
    OK(String),
}

#[derive(Debug)]
pub struct ScanStrategyResult {
    pub strategy: String,
    pub result: FileStatus,
}
