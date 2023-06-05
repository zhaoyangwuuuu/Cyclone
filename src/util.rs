use std::io::{self, BufReader, Read, Write};

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
