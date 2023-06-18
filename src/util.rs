use std::fs;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};

/// Humanize bytes into a more readable format
pub fn humanize_bytes(bytes: u64) -> String {
    let values = ["bytes", "KB", "MB", "GB", "TB"];
    let pair = values
        .iter()
        .enumerate()
        .take_while(|x| bytes / 1000_u64.pow(x.0 as u32) > 10)
        .last();
    if let Some((i, unit)) = pair {
        format!("{} {}", bytes / 1000_u64.pow(i as u32), unit)
    } else {
        format!("{} {}", bytes, values[0])
    }
}

/// Prompt for user input, returning True if the first character is 'y' or 'Y'
pub fn prompt_yes<T: AsRef<str>>(prompt: T) -> bool {
    print!("{} (y/N) ", prompt.as_ref());
    if io::stdout().flush().is_err() {
        // If stdout wasn't flushed properly, fallback to println
        println!("{} (y/N)", prompt.as_ref());
    }
    let stdin = BufReader::new(io::stdin());
    stdin
        .bytes()
        .next()
        .and_then(|c| c.ok())
        .map(|c| c as char)
        .map(|c| (c == 'y' || c == 'Y'))
        .unwrap_or(false)
}

/// Concatenate two paths, even if the right argument is an absolute path.
pub fn join_absolute<A: AsRef<Path>, B: AsRef<Path>>(left: A, right: B) -> PathBuf {
    let (left, right) = (left.as_ref(), right.as_ref());
    left.join(if let Ok(stripped) = right.strip_prefix("/") {
        stripped
    } else {
        right
    })
}

/// Check if a symlink exists at the given path.
pub fn symlink_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::symlink_metadata(path).is_ok()
}

/// Add a numbered extension to duplicate filenames to avoid overwriting files.
pub fn rename_tempfile<G: AsRef<Path>>(file: G) -> PathBuf {
    let file = file.as_ref();
    let name = file.to_str().expect("Filename must be valid unicode.");
    (1_u64..)
        .map(|i| PathBuf::from(format!("{}~{}", name, i)))
        .find(|p| !symlink_exists(p))
        .expect("Failed to rename duplicate file or directory")
}
