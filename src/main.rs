mod args;
mod confiq;
mod utils;
mod error_handling;
mod commands;

use std::fs::File;

use args::Args;
use clap::Parser;
use simplelog::*;
use commands::*;

pub use error_handling::{Error, Result};

fn main() {
    let args = Args::parse();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];

    if args.global_opts.verbose {
        loggers.push(TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ));
    }

    if args.global_opts.log_to_file {
        loggers.push(WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(std::env::temp_dir().join("confiq.log")).unwrap(),
        ));
    }

    CombinedLogger::init(loggers).unwrap();

    match args.command {
        args::Command::Build(args) => {
            build(args);
        }
        args::Command::Scopes(args) => {
            scope(args);
        }
        args::Command::List(args) => {
            list(args);
        }
    }
}
