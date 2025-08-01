pub fn new() -> String {
    match std::env::current_dir() {
        Ok(dir) => dir.to_string_lossy().into_owned(),
        Err(err) => panic!("Failed to get current directory: {}", err),
    }
}