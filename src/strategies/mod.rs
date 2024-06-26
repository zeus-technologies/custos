mod hash_strategy;

pub use hash_strategy::SHA256FileScanStrategy;

pub trait ScanStrategy {
    fn get_name(&self) -> &str;
    fn process(&self, path: &std::path::Path, data: &[u8]) -> ScanStrategyResult;
    fn update(&self, path: &std::path::Path, data: &[u8]);
}

pub enum ScanStrategyResult {
    NewFile(String),
    UpdatedFile(String),
    MaliciousFile(String),
    OK,
}
