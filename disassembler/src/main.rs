use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(about)]
struct Cli {
    /// File(s) to disassemble
    files: Vec<PathBuf>,
}

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
