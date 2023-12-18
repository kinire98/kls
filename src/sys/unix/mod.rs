/*

For linux there is a crate file_mode:
https://docs.rs/file-mode/latest/file_mode/
https://crates.io/crates/file-mode
Saves me
*/



use file_mode::{ModePath, User};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::path::Path;

pub struct MetadataPath {
    path: PathBuf,
}


impl MetadataPath {
    pub fn from(path: PathBuf) -> Self {
        MetadataPath { path }
    }
    pub fn size(&self) -> u64 {
        fs::metadata(self.path.clone()).unwrap().size()
    }
    pub fn props(&self) -> String {
        let mode = Path::new(&self.path).mode().unwrap();
        let permissions = mode.to_string();
        permissions
    }
}