//! A disassembler for Intel 8080 programs.
//!
//! A CLI driver for the `parser` library.
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(about)]
/// Arguments parsed from the command line
struct Cli {
    /// File(s) to disassemble
    files: Vec<PathBuf>,
}

#[expect(clippy::print_stdout, reason = "Actual CLI output")]
fn main() {
    let args = Cli::parse();

    for file in args.files {
        println!("Operating on {}", file.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Assert there are no clap errors
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
