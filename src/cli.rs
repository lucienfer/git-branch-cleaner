use clap::Parser;
use std::io::{self, Write};

use crate::utils::error_cli;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Don't actually delete anything
    #[arg(short, long)]
    pub dry_run: bool,

    /// Force deletion without asking
    #[arg(short, long)]
    pub force: bool,

    /// Base branch to compare against
    #[arg(long, default_value = "main")]
    pub base: String,
}

/// Request confirmation to delete branch
///
/// # Argument
/// - `prompt` - &str
///
/// # Return
/// Return if the caller has confirm the delete or not
pub fn confirm(prompt: &str) -> bool {
    print!("{prompt} [Y/N]");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let good_input = matches!(
        input.trim().to_lowercase().as_str(),
        "y" | "yes" | "n" | "no"
    );
    if !good_input {
        error_cli("Enter a good input: y | yes | no | n");
        confirm(prompt)
    } else {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }
}
