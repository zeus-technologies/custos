use clap::{Parser, ValueEnum};
use reports::terminal_reporter::TerminalReporter;
use tracing::{debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod coordinator;
mod db;
mod reports;
mod strategies;

#[derive(Parser, Debug)]
#[command(version, name = "custos")]
struct Cli {
    /// The mode to run custos in, either scan or update
    #[arg(value_enum)]
    mode: RunMode,
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, ValueEnum)]
enum RunMode {
    Scan,
    Update,
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("error")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();
    let config = config::get();
    let args = Cli::parse();

    // run the migrations
    let pool = db::database::get_connection_pool();
    let mut conn = pool.get().expect("get connection");
    db::file_repository::run_migrations(&mut conn).expect("ran migrations");

    // create the reporter
    let (sender, receiver) = crossbeam::channel::unbounded();
    tokio::spawn(async move {
        let mut reporter = reports::ReportManager::new(receiver);
        reporter.add_reporter(Box::new(TerminalReporter::new(args.quiet)));
        reporter.process_results();
    });

    // scan the directories
    for directory in &config.scan_directories {
        debug!("scanning directory: {}", directory);

        let entries = match glob::glob(directory) {
            Ok(entries) => entries,
            Err(e) => {
                error!("failed to read glob pattern: {:?}", e);
                continue;
            }
        }
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.as_os_str().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

        let chunk_size = entries.len() / config.thread_count;
        let chunks = entries.chunks(chunk_size).map(|chunk| chunk.to_vec());
        let mut workers = Vec::new();

        let update = args.mode == RunMode::Update;
        for chunk in chunks {
            let s = sender.clone();
            workers.push(tokio::spawn(async move {
                let mut scanner = coordinator::ScanCoordinator::new(update, &chunk, s);
                scanner.add_scan_strategy(Box::new(strategies::SHA256FileScanStrategy::new()));
                scanner.run();
            }));
        }

        for w in workers {
            if let Err(e) = w.await {
                error!("worker failed: {:?}", e);
            }
        }
    }
}
