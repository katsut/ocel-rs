use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use ocel::io;

/// OCEL 2.0 command-line tools.
#[derive(Debug, Parser)]
#[command(name = "ocel", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert an OCEL 2.0 log between formats (chosen by file extension).
    Convert {
        /// Input file (.json / .jsonocel, .sqlite / .db, .xml / .xmlocel).
        input: PathBuf,
        /// Output file (format chosen by its extension).
        output: PathBuf,
    },
    /// Validate an OCEL 2.0 log against the specification.
    Validate {
        /// File to validate.
        input: PathBuf,
    },
}

fn run(cli: Cli) -> Result<ExitCode, Box<dyn Error>> {
    match cli.command {
        Command::Convert { input, output } => {
            let ocel = io::read_path(&input)?;
            io::write_path(&ocel, &output)?;
            println!("converted {} -> {}", input.display(), output.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Validate { input } => {
            let ocel = io::read_path(&input)?;
            match ocel.validate() {
                Ok(()) => {
                    println!("valid: {}", input.display());
                    Ok(ExitCode::SUCCESS)
                }
                Err(violations) => {
                    eprintln!("{} violation(s) in {}:", violations.len(), input.display());
                    for violation in &violations {
                        eprintln!("  - {violation}");
                    }
                    Ok(ExitCode::FAILURE)
                }
            }
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
