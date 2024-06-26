use tracing::{debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod coordinator;
mod db;
mod strategies;

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

        for chunk in chunks {
            workers.push(tokio::spawn(async move {
                let scanner = coordinator::ScanCoordinator::new(&chunk);
                scanner.run();
            }));
        }

        for w in workers {
            if let Err(e) = w.await {
                error!("worker failed: {:?}", e);
            }
        }
    }
    // let compiler = Compiler::new().unwrap();
    // let compiler = compiler
    //     .add_rules_file(Path::new("rules/rust.yar"))
    //     .expect("parsed rules file");
    // let rules = compiler.compile_rules().expect("compiled rules");
    // let results = rules
    //     .scan_mem("I love Rust!".as_bytes(), 5)
    //     .expect("scan memory");
    // assert!(results.iter().any(|x| x.identifier == "contains_rust"));
}
