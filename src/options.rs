use std::fs;

pub fn delete(file: &str, tempstore: &str) {
    // Check if source exists
    if let Ok(metadata) = fs::symlink_metadata(file) {
        let source = &if !metadata.file_type().is_symlink() {
            file
        } else {
            todo!()
        };
    }
}
