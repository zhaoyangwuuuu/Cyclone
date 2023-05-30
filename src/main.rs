#[macro_use]
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate time;
extern crate walkdir;

use clap::{Args, Parser, Subcommand};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use walkdir::WalkDir;
mod errors {
    error_chain! {}
}
use errors::*;

const Tempstore: &str = "/tmp/tempstore";
const RECORD: &str = ".record";

struct RecordItem<'a> {
    _time: &'a str,
    orig: &'a Path,
    dest: &'a Path,
}

#[derive(Parser)]
#[command(author = "Zhaoyang Wu", version)]
#[command(about = "cyclone - a CLI alternative to rm")]
struct Cli {
    name: String,

    /// Directory where deleted files reside before being permanently deleted
    #[arg(short = 't', long = "tempstore")]
    tempstore: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {}

#[derive(Args)]
struct List {}

#[derive(Args)]
struct Restore {}

#[derive(Args)]
struct Preview {}
fn main() {
    run();
}

fn run() {
    let cli = Cli::parse();
    if let Some(tempstore) = cli.tempstore {
        println!("tempstore: {}", tempstore);
    }

    let name = cli.name;
}
