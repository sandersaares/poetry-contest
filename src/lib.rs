use std::path::PathBuf;

pub mod v01_naive;

pub fn find_workspace_root() -> PathBuf {
    let mut current = std::env::current_dir().expect("Failed to get current directory");
    loop {
        if current.join("Cargo.toml").exists() {
            return current;
        }
        if !current.pop() {
            panic!("Could not find workspace root (Cargo.toml)");
        }
    }
}
