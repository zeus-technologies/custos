use sha2::{Digest, Sha256};
use tracing::{debug, error, trace};

use crate::db;

use super::ScanStrategy;

#[derive(Debug)]
pub struct SHA256FileScanStrategy;

impl SHA256FileScanStrategy {
    pub fn new() -> SHA256FileScanStrategy {
        SHA256FileScanStrategy {}
    }

    pub fn calculate_hash(&self, file: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(file);
        hasher.finalize().to_vec()
    }
}

impl ScanStrategy for SHA256FileScanStrategy {
    fn process(&self, path: &std::path::Path, data: &[u8]) {
        let hash = self.calculate_hash(&data);
        let mut pool = db::database::get_connection_pool();
        if let Some(file_history) = db::file_repository::get_file(&mut pool, path.to_str().unwrap())
        {
            trace!("file already exists, comparing hashes!");
            if hash.as_slice() == hex::decode(file_history.hash).unwrap() {
                trace!("file hashes matches, file is unchanged.");
                return;
            } else {
                error!("file hashes do not match, raising an error!");
            }
            debug!("{:?}: {:?}", path, hash);
        } else {
            debug!("file does not exist, inserting!");
            db::file_repository::insert_file(
                &mut pool,
                db::file_repository::File {
                    filepath: path.to_str().unwrap().to_string(),
                    hash: hex::encode(hash),
                },
            );
        }
    }
}
