use anyhow::{Context, Result};
use std::env;
use std::fs::{self, Metadata};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::util::{
    copy_file, humanize_bytes, join_absolute, prompt_yes, rename_tempfile, symlink_exists,
    write_log,
};
use crate::Cli;

const FILES_TO_INSPECT: usize = 6;
const LINES_TO_INSPECT: usize = 6;
const DEFAULT_TEMPSTORE: &str = "/tmp/tempstore";
const RECORD: &str = ".record";

struct RecordItem<'a> {
    _time: &'a str,
    orig: &'a Path,
    dest: &'a Path,
}

pub fn delete(file: &str, cli: &Cli) -> Result<()> {
    let cwd: PathBuf = env::current_dir().context("Failed to get current dir")?;

    // Check if source exists
    if let Ok(metadata) = fs::symlink_metadata(file) {
        let source = &if !metadata.file_type().is_symlink() {
            cwd.join(file)
                .canonicalize()
                .context("Failed to canonicalize path")?
        } else {
            cwd.join(file)
        };

        // Check if preview is enabled
        if cli.preview {
            preview(&metadata, source, file);
        }

        let tempstore: PathBuf = cli
            .tempstore
            .clone()
            .unwrap_or_else(|| match env::var("TEMPSTORE") {
                Ok(val) => val,
                Err(_) => DEFAULT_TEMPSTORE.to_string(),
            })
            .into();
        let record: &Path = &tempstore.join(RECORD);

        println!("tempstore: {:?}", tempstore);

        let dest: &Path = &{
            let dest = join_absolute(tempstore, source);
            // Resolve a name conflict if necessary
            if symlink_exists(&dest) {
                rename_tempfile(dest)
            } else {
                dest
            }
        };

        // Move the file to the tempstore
        if fs::rename(source, dest).is_ok() {
            return Ok(());
        }

        // If the move failed, try copying the file instead
        let parent = dest.parent().context("Couldn't get parent of dest")?;
        // Create the parent directory if it doesn't exist
        fs::create_dir_all(parent).context("Couldn't create parent dir")?;

        // If the source is a directory, copy it recursively
        if fs::symlink_metadata(source)
            .context("Couldn't get metadata")?
            .is_dir()
        {
            // Walk the source, creating directories and copying files as needed
            for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
                // Path without the top-level directory
                let orphan: &Path = entry
                    .path()
                    .strip_prefix(source)
                    .context("Parent directory isn't a prefix of child directories?")?;
                if entry.file_type().is_dir() {
                    fs::create_dir_all(dest.join(orphan)).context({
                        format!(
                            "Failed to create {} in {}",
                            entry.path().display(),
                            dest.join(orphan).display()
                        )
                    })?;
                } else {
                    copy_file(entry.path(), dest.join(orphan)).context({
                        format!(
                            "Failed to copy file from {} to {}",
                            entry.path().display(),
                            dest.join(orphan).display()
                        )
                    })?;
                }
            }
            fs::remove_dir_all(source)
                .context(format!("Failed to remove dir: {}", source.display()))?;
        } else {
            copy_file(source, dest).context({
                format!(
                    "Failed to copy file from {} to {}",
                    source.display(),
                    dest.display()
                )
            })?;
            fs::remove_file(source)
                .context(format!("Failed to remove file: {}", source.display()))?;
        }
        write_log(source, dest, record)
            .context(format!("Failed to write record at {}", record.display()))?;
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

    if !prompt_yes(format!("Delete this file {}?", file)) {
        todo!();
    }
}

pub fn list() -> Result<()> {
    let cwd: PathBuf = env::current_dir().context("Failed to get current dir")?;
    println!("cwd: {:?}", cwd);

    let tempstore = cwd.join(".tempstore");
    println!("tempstore: {:?}", tempstore);

    if !tempstore.exists() {
        println!("No files to list");
        return Ok(());
    }

    for entry in WalkDir::new(tempstore)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        println!("{}", entry.path().display());
    }

    Ok(())
}
