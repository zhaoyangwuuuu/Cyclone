pub fn humanize_bytes(bytes: u64) -> String {
    let values = ["bytes", "KB", "MB", "GB", "TB"];
    let pair = values
        .iter()
        .enumerate()
        .take_while(|x| bytes as usize / (1000 as usize).pow(x.0 as u32) > 10)
        .last();
    if let Some((i, unit)) = pair {
        format!(
            "{} {}",
            bytes as usize / (1000 as usize).pow(i as u32),
            unit
        )
    } else {
        format!("{} {}", bytes, values[0])
    }
}
