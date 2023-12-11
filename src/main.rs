use std::{fs, path::PathBuf};
#[cfg(windows)]
use std::os::windows::prelude::*;
#[cfg(unix)]
use std::os::linux::fs::MetadataExt;
use clap::*;

use std::env::current_dir;

#[derive(Parser)]
struct Args {
    path: Option<PathBuf>
}



fn main() {
    let args = Args::parse();
    let mut path = if let Some(x) = args.path {
        x
    } else {
        PathBuf::new()
    };
    if path == PathBuf::new() {
        path = current_dir().unwrap();
    }
    println!("{:?}", path);
    
    let thing = std::fs::read_dir(path).unwrap();
    for path in thing {
        let jeje = path.unwrap().path().display().to_string();
        let metadata = fs::metadata(jeje.clone()).unwrap();
        #[cfg(windows)]
        {
            let atts = metadata.file_attributes();
            let file_size = metadata.file_size();
            println!("{}: {:?}, {:?}",jeje,  atts, file_size);
        }
        #[cfg(unix)]
        {
            let atts = metadata.file_type();
            let file_size = metadata.st_blocks();
            println!("{}: {:?}, {:?}",jeje,  atts, file_size);
        }
    }
}
