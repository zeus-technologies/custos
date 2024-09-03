use super::{FileStatus, ProcessStrategy};

pub struct YaraFileScanStrategy {
    rules: yara::Rules,
}

impl YaraFileScanStrategy {
    pub fn new(rule_directory: String) -> YaraFileScanStrategy {
        let mut compiler = yara::Compiler::new().unwrap();
        let paths = std::fs::read_dir(rule_directory).unwrap();
        for path in paths {
            compiler = compiler
                .add_rules_file(path.unwrap().path())
                .expect("add yara rule file");
        }
        YaraFileScanStrategy {
            rules: compiler.compile_rules().expect("yara rule compilation"),
        }
    }
}

impl ProcessStrategy for YaraFileScanStrategy {
    fn process(
        &self,
        status: &super::FileStatus,
        path: &std::path::Path,
        data: &[u8],
    ) -> Option<super::FileStatus> {
        if let FileStatus::ReadFailed(_, _) = status {
            return None;
        }

        let matches = self.rules.scan_mem(data, 30).expect("yara scan");
        if matches.len() > 0 {
            Some(FileStatus::MaliciousFile(
                path.to_str().unwrap().to_string(),
            ))
        } else {
            None
        }
    }

    fn get_name(&self) -> &str {
        "YaraFileScanStrategy"
    }
}
