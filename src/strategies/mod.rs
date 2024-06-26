mod hash_strategy;

pub use hash_strategy::SHA256FileScanStrategy;

pub trait ScanStrategy {
    fn process(&self, path: &std::path::Path, data: &[u8]);
}
