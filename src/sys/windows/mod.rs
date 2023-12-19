/*

? Docs for attributes constants for files in Windows
File attribute constant: 
https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/file-attribute-constants


Const attributes:
1 -> readonly
2 -> hidden
4 -> system file
16 -> is dir
32 -> file
128 -> normal xD


It uses binary bits for each attribute
for example if is a hidden dir it would be:
is dir -> 16 + hidden -> 2 = 18 -> hidden dir
128 cannot be mixed



https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.file_attributes
*/

use bit_vec::BitVec;

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;

use nt_time::{ 
    FileTime,
    time::OffsetDateTime
};

use std::{fs, time::{
    Duration,
    UNIX_EPOCH
}};
use std::os::windows::prelude::*;
use std::path::PathBuf;



pub struct MetadataPath {
    path: PathBuf,
}


impl MetadataPath {
    pub fn from(path: PathBuf) -> Self {
        MetadataPath { path }
    }
    pub fn size(&self) -> u64 {
        fs::metadata(self.path.clone()).unwrap().file_size()
    }
    pub fn props(&self) -> String {
        let metadata = fs::metadata(self.path.clone()).unwrap();
        let file_props = metadata.file_attributes();
        let props_array =  BitVec::from_elem(file_props.try_into().unwrap(), false);
        let last_write_time = Self::get_date(metadata.last_write_time()); // 100 nanoseconds intervals since January 1, 1601 (UTC)
        format!("{} {} {}", "", last_write_time, "")
    }
    fn get_date(write_time: u64) -> String {
        /*
        ! The 10_000_000 constant exists because windows stores the time since its own EPOCH
        ! similar to Unix but with January 1 1601 00:00:00 but it doesn't give you seconds but
        ! 100 nanoseconds intervals. To convert it to seconds divide between 10_000_000
        ! Expect losses in precision, more specifically one hour less, that's why the + 3600 is there
         */
        let write_time_seconds = write_time / 10_000_000 + 3600;
        let windows_time = FileTime::NT_TIME_EPOCH + Duration::from_secs(write_time_seconds);
        let unix_time = windows_time.to_unix_time();
        let unix_time_with_epoch = UNIX_EPOCH + Duration::from_secs(unix_time.try_into().unwrap());
        let datetime = DateTime::<Utc>::from(unix_time_with_epoch);
        datetime.format("%Y-%m-%d %H:%M").to_string()
    }
}