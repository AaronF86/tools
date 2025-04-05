//------------------------------------------//
//                                          //
// main.rs - main entry point               //
//                                          //
//------------------------------------------//


mod banner;
mod cli;
mod config;
mod file_handler;
mod language;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    if let Err(err) = config::initialize_config() {
        eprintln!("Failed to initialize config: {}", err);
        std::process::exit(1);
    }

    let cli = Cli::parse();
    cli.run();
}