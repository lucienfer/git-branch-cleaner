mod cleaner;
mod cli;
mod git;
mod utils;

use clap::Parser;
use cli::CliArgs;
use std::process::Command;
use utils::{error_cli, success};

fn main() {
    let args = CliArgs::parse();
    let output = Command::new("echo")
        .arg("Hello, world")
        .output()
        .expect("Failed to execute echo");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        success(&stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error_cli(&stderr);
    }
}
