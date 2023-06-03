use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::util::humanize_bytes;
use crate::Cli;

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
        if cli.preview && metadata.is_dir() {
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
        }
    }

    Ok(())
}
