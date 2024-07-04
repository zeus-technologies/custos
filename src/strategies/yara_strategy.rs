use super::ProcessStrategy;

pub struct YaraFileScanStrategy {
    rule_directory: String,
}

impl YaraFileScanStrategy {
    pub fn new(rule_directory: String) -> YaraFileScanStrategy {
        YaraFileScanStrategy { rule_directory }
    }
}

impl ProcessStrategy for YaraFileScanStrategy {
    fn process(
        &self,
        status: super::FileStatus,
        path: &std::path::Path,
        data: &[u8],
    ) -> super::FileStatus {
        status
    }

    fn get_name(&self) -> &str {
        "YaraFileScanStrategy"
    }
}
