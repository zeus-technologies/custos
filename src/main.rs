use clap::{Parser, ValueEnum};
use tracing::{debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod coordinator;
mod db;
mod strategies;

#[derive(Parser, Debug)]
#[command(version, name = "aegis")]
struct Cli {
    /// The mode to run aegis in, either scan or update
    #[arg(value_enum)]
    mode: RunMode,
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
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
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
            workers.push(tokio::spawn(async move {
                let scanner = coordinator::ScanCoordinator::new(update, &chunk);
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
