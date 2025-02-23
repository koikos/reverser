pub fn validate_path(path: &Path) -> bool {
    if !path.exists() || !path.is_file() {
        eprintln!("Error: File not found or not a valid file.");
        return false;
    }
    true
}