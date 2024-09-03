use tracing::warn;

use super::ProcessStrategy;

pub struct YaraFileScanStrategy {
   rules: Option<yara::Rules>
}

impl YaraFileScanStrategy {
    pub fn new(rule_directory: String) -> YaraFileScanStrategy {
        let mut compiler = yara::Compiler::new().unwrap();
        let paths = std::fs::read_dir(rule_directory).unwrap();
        for path in paths {
            compiler = compiler.add_rules_file(path.unwrap().path()).expect("add yara rule file");
        }
        YaraFileScanStrategy { 
            rules: Some(compiler.compile_rules().expect("yara rule compilation")),
        }
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
