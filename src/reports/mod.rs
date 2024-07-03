use crate::strategies;

pub use report_manager::ReportManager;

mod report_manager;
pub mod terminal_reporter;

pub(crate) trait Reporter {
    fn report(&self, result: &strategies::ScanStrategyResult);
}
