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
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Graveyard(Graveyard),
}

#[derive(Args)]
struct Graveyard {}

#[derive(Args)]
struct Decompose {}

#[derive(Args)]
struct Seance {}

#[derive(Args)]
struct Unbury {}

#[derive(Args)]
struct Inspect {}
fn main() {}
