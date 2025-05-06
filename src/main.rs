mod cleaner;
mod cli;
mod git;
mod utils;

use clap::Parser;
use cleaner::run;
use cli::CliArgs;
use utils::{error_cli, success};

fn main() {
    let args = CliArgs::parse();
    match run(args) {
        Ok(()) => success("Branch cleaning finished successfully."),
        Err(e) => error_cli(&e.to_string()),
    };
}
