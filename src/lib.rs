use std::path::PathBuf;

pub mod v01_naive;
pub mod v02_borrow_document;
pub mod v03_borrow_more;
pub mod v04_reserve_and_reuse;
pub mod v05_raw_contents;
pub mod v06_reuse_more;
pub mod v07_frozen;

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
