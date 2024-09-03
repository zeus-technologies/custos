mod hash_strategy;
mod yara_strategy;

use std::io::Error;

pub use hash_strategy::SHA256FileScanStrategy;
pub use yara_strategy::YaraFileScanStrategy;

pub trait ScanStrategy {
    fn get_name(&self) -> &str;
    fn process(&self, path: &std::path::Path, data: &[u8]) -> FileStatus;
    fn update(&self, path: &std::path::Path, data: &[u8]);
}

pub trait ProcessStrategy {
    fn process(
        &self,
        status: &FileStatus,
        path: &std::path::Path,
        data: &[u8],
    ) -> Option<FileStatus>;
    fn get_name(&self) -> &str;
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
pub struct StrategyResult {
    pub strategy: String,
    pub result: FileStatus,
}
