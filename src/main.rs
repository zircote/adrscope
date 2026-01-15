//! ADRScope CLI binary entry point.

use clap::Parser;

use adrscope::cli::{Cli, run};

fn main() {
    let cli = Cli::parse();

    match run(cli) {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        },
    }
}
