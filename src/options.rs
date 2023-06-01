use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn delete(file: &str, tempstore: &str) -> Result<()> {
    let cwd: PathBuf = env::current_dir().context("Failed to get current dir")?;

    // Check if source exists
    if let Ok(metadata) = fs::symlink_metadata(file) {
        let source = &if !metadata.file_type().is_symlink() {
            cwd.join(file)
                .canonicalize()
                .context("Failed to canonicalize path")?
        } else {
            todo!()
        };
    }

    Ok(())
}
