use anyhow::{Context, Result};
use std::env;
use std::fs::{self, Metadata};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::util::humanize_bytes;
use crate::Cli;

const FILES_TO_INSPECT: usize = 6;
const LINES_TO_INSPECT: usize = 6;

pub fn delete(file: &str, cli: &Cli) -> Result<()> {
    let cwd: PathBuf = env::current_dir().context("Failed to get current dir")?;
    println!("cwd: {:?}", cwd);

    // Check if source exists
    if let Ok(metadata) = fs::symlink_metadata(file) {
        let source = &if !metadata.file_type().is_symlink() {
            cwd.join(file)
                .canonicalize()
                .context("Failed to canonicalize path")?
        } else {
            cwd.join(file)
        };
        println!("source: {:?}", source);

        // Check if preview is enabled
        if cli.preview {
            preview(&metadata, source, file);
        }
    }

    Ok(())
}

// Preview the changes without actually deleting the files
fn preview(metadata: &Metadata, source: &PathBuf, file: &str) {
    if metadata.is_dir() {
        // Get the size of the directory and all its contents
        println!(
            "{}: directory, {} including:",
            file,
            humanize_bytes(
                WalkDir::new(source)
                    .into_iter()
                    .filter_map(|x| x.ok())
                    .filter_map(|x| x.metadata().ok())
                    .map(|x| x.len())
                    .sum::<u64>()
            )
        );

        // Print the first few files in the directory
        for entry in WalkDir::new(source)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .take(FILES_TO_INSPECT)
        {
            println!("{}", entry.path().display());
        }
    } else {
        println!("{}: file, {}", file, humanize_bytes(metadata.len()));
        // Read the file and print the first few lines
        if let Ok(f) = fs::File::open(source) {
            for line in BufReader::new(f)
                .lines()
                .take(LINES_TO_INSPECT)
                .filter_map(|line| line.ok())
            {
                println!("> {}", line);
            }
        } else {
            println!("Error reading {}", source.display());
        }
    }
}
