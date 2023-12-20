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

This are the ones that Get-ChildItem displays
d - Directory -> 16 -> bit 5
a - Archive -> 32 -> bit 6
r - Read-only -> 1 -> bit 1
h - Hidden -> 2 -> bit 2
s - System -> 4 -> bit 3
l - Reparse point, symlink, etc. -> 512 -> bit 10



These are all that windows stores
ReadOnly
Hidden
System
Directory
Archive
Device
Normal
Temporary
SparseFile
ReparsePoint
Compressed
Offline
NotContentIndexed
Encrypted
(The same as in this link:
    https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
    From 1 to 16384
    Windows stores more but are more system specific, even some that in docs
    says explicitly that you should not use them
)
What interest me to implement

https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.file_attributes
*/


extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;

use nt_time::FileTime;

use std::{fs, time::{
    Duration,
    UNIX_EPOCH
}, fmt::Display};
use std::os::windows::prelude::*;
use std::path::PathBuf;



pub struct MetadataPath {
    path: PathBuf,
}
struct Props {
    dir: bool, 
    file: bool,
    read_only: bool,
    hidden: bool,
    system: bool,
    reparse_file: bool
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
        let file_props = Self::props_string(metadata.file_attributes());
        let last_write_time = Self::get_date(metadata.last_write_time());
        format!("{} {}", file_props, last_write_time)
    }
    fn get_date(write_time: u64) -> String {
        /*
        ! The 10_000_000 constant exists because windows stores the time since its own EPOCH
        ! similar to Unix but with January 1 1601 00:00:00 but it doesn't give you seconds but
        ! 100 nanoseconds intervals. To convert it to seconds divide between 10_000_000
        ! Expect losses in precision, more specifically one hour less (and probably some seconds but in this use case seconds are not relevant),
        ! that's why the + 3600 is there
         */
        let write_time_seconds = write_time / 10_000_000 + 3600;
        let windows_time = FileTime::NT_TIME_EPOCH + Duration::from_secs(write_time_seconds);
        let unix_time = windows_time.to_unix_time();
        let unix_time_with_epoch = UNIX_EPOCH + Duration::from_secs(unix_time.try_into().unwrap());
        let datetime = DateTime::<Utc>::from(unix_time_with_epoch);
        datetime.format("%Y-%m-%d %H:%M").to_string()
    }
    fn props_string(props: u32) -> String {
        Props::new(props).to_string()
    }
}

impl Props {
    fn new(props: u32) -> Self {
        let mut clone = props;
        let mut props = props;
        let mut read_only = false;
        props >>= 1;
        props <<= 1;
        if props != clone {
            read_only = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut hidden = false;
        props >>= 1;
        props <<= 1;
        if props != clone {
            hidden = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut system = false;
        props >>= 1;
        props <<= 1;
        if props != clone {
            system = true;
        }
        props >>= 2;
        clone >>= 2;
        let mut dir = false;
        props >>= 1;
        props <<= 1;
        if props != clone {
            dir = true;
        }
        props >>= 1;
        clone >>= 1;
        let mut file = false; 
        props >>= 1;
        props <<= 1;
        if props != clone && !dir {
            file = true;
        }
        props >>= 5;
        clone >>= 5;
        let mut reparse_file = false;
        props >>= 1;
        props <<= 1;
        if props != clone {
            reparse_file = true;
        }
        Props { dir, file, read_only, hidden, system, reparse_file }
    }
}
impl Display for Props {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}", if self.dir {'d'} else {'-'},
                if self.file {'a'} else {'-'},
                if self.read_only {'r'} else {'-'},
                if self.hidden {'h'} else {'-'},
                if self.system {'s'} else {'-'},
                if self.reparse_file {'l'} else {'-'})
    }
}