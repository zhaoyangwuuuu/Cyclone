#[macro_use]
extern crate core;
extern crate time;
extern crate walkdir;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::Path;

mod options;
mod util;

const RECORD: &str = ".record";

struct RecordItem<'a> {
    _time: &'a str,
    orig: &'a Path,
    dest: &'a Path,
}

#[derive(Parser)]
#[command(author = "Zhaoyang Wu", version)]
#[command(about = "cyclone - a CLI alternative to rm")]
pub struct Cli {
    #[arg(num_args=1..)]
    files: Vec<String>,

    /// Directory where deleted files reside before being permanently deleted
    #[arg(short = 't', long = "tempstore")]
    tempstore: Option<String>,

    /// Preview the changes without actually deleting the files
    #[arg(short = 'p', long = "preview")]
    preview: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all files in the tempstore
    List(List),
}

#[derive(Args)]
struct List {
    /// List all files in the tempstore
    #[arg(short = 'a', long = "all", default_value = "true")]
    all: bool,

    #[arg(short = 's', long = "single")]
    single: Option<String>,
}

#[derive(Args)]
struct Restore {}

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    for file in &cli.files {
        println!("{}", file);
        options::delete(file, &cli)?;
    }

    match &cli.command {
        Some(Commands::List(list)) => {
            println!("{:?}", list.all);
            options::list()?;
            if let Some(single) = &list.single {
                todo!();
            }
        }
        None => {
            println!("Please provide a string to inspect");
        }
    }

    Ok(())
}
