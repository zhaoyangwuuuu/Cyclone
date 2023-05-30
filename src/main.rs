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

#[derive(Parser)]
#[command(author = "Zhaoyang", about, version)]
struct Cli {
    #[arg()]
    name: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Hold(Hold),
}

#[derive(Args)]
struct Hold {
    path: PathBuf,
}

#[derive(Args)]
struct Delete {}

#[derive(Args)]
struct List {}

#[derive(Args)]
struct Restore {}

#[derive(Args)]
struct Preview {}
fn main() {
    let cli = Cli::parse();

    println!("name: {}", cli.name);
}
