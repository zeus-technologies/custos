# Custos [![dependency status](https://deps.rs/repo/github/zeus-technologies/custos/status.svg)](https://deps.rs/repo/github/zeus-technologies/custos) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/zeus-technologies/custos)](https://rust-reportcard.xuri.me/report/github.com/zeus-technologies/custos)


Custos is a Rust-based Intrusion Detection System (IDS). This applications scans configured directories, hashes files and stores the results to detect changes to files as well as new files. These new or changed files can then be processed using YARA rules to detect files with malicious signatures.

## Features
- Directory Scanning
- File Hashing
- YARA Rule Integration

## Installation
### Prerequisites
- Rust
- Diesel CLI
- YARA

### Steps
1. Clone the repository
2. Run `diesel setup` to setup the database
3. Run `cargo build --release` to build the application
4. Run `./target/release/custos -h` to start the application

### Configuration
The configuration file is located at `custos.toml`. An example cofiguration is located within this repository.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
