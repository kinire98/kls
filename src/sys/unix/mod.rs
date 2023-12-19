/*

For linux there is a crate file_mode:
https://docs.rs/file-mode/latest/file_mode/
https://crates.io/crates/file-mode
https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html
Saves me
*/

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;

use file_mode::ModePath;



use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::path::Path;
use std::time::{UNIX_EPOCH, Duration};
use std::process::Command;

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
        let meta = fs::metadata(self.path.clone()).unwrap();
        let gid = Self::parse_passwd_gid(meta.gid());
        let uid = Self::parse_passwd_uid(meta.uid());
        let date = Self::get_date(meta.mtime().try_into().unwrap());
        format!("{} {} {} {}", permissions, uid, gid, date)
    }
    fn parse_passwd_uid(uid: u32) -> String {
        let etc_passwd = fs::read_to_string("/etc/passwd").unwrap();
        let mut result = "";
        etc_passwd.split('\n').collect::<Vec<&str>>().iter().rev().for_each(|n| {
            let line = n.split(':').collect::<Vec<&str>>();
            if line.len() < 2 { return; }
            if line[2].parse::<u32>().unwrap() == uid {
                result = line[0];
            }
        });
        if result.is_empty() {
            String::from("Unknown")
        } else {
            String::from(result)
        }
    }
    fn parse_passwd_gid(gid: u32) -> String {
        let output = String::from_utf8_lossy(&Command::new("getent")
                                                        .arg("group")
                                                        .arg(gid.to_string())
                                                        .output()
                                                        .expect("Command failed to start")
                                                        .stdout)
                                                        .to_string();
        output.split(':').collect::<Vec<&str>>()[0].to_string()
    }
    fn get_date(unix_time: u64) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(unix_time);
        let datetime = DateTime::<Utc>::from(d);
        datetime.format("%Y-%m-%d %H:%M").to_string()
    }
}