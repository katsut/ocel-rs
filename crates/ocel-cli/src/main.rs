use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use ocel::io::{json, sqlite, xml};
use ocel::Ocel;

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
        /// Input file (.json / .jsonocel, .sqlite, .xml / .xmlocel).
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

#[derive(Debug, Clone, Copy)]
enum Format {
    Json,
    Sqlite,
    Xml,
}

fn detect(path: &Path) -> Result<Format, Box<dyn Error>> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "json" | "jsonocel" => Ok(Format::Json),
        "sqlite" | "db" => Ok(Format::Sqlite),
        "xml" | "xmlocel" => Ok(Format::Xml),
        other => Err(format!("unknown file extension: {other:?}").into()),
    }
}

fn read_any(path: &Path) -> Result<Ocel, Box<dyn Error>> {
    match detect(path)? {
        Format::Json => Ok(json::read_path(path)?),
        Format::Sqlite => Ok(sqlite::read_path(path)?),
        Format::Xml => Ok(xml::read_path(path)?),
    }
}

fn write_any(ocel: &Ocel, path: &Path) -> Result<(), Box<dyn Error>> {
    match detect(path)? {
        Format::Json => json::write_path(ocel, path)?,
        Format::Sqlite => sqlite::write_path(ocel, path)?,
        Format::Xml => xml::write_path(ocel, path)?,
    }
    Ok(())
}

fn run(cli: Cli) -> Result<ExitCode, Box<dyn Error>> {
    match cli.command {
        Command::Convert { input, output } => {
            let ocel = read_any(&input)?;
            write_any(&ocel, &output)?;
            println!("converted {} -> {}", input.display(), output.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Validate { input } => {
            let ocel = read_any(&input)?;
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
