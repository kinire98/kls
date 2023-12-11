use std::fs;
#[cfg(windows)]
use std::os::windows::prelude::*;
#[cfg(unix)]
use std::os::linux::fs::MetadataExt;
use clap::*;

#[derive(Parser)]
struct Args {
    path: String
}



fn main() {
    let args = Args::parse();
    let thing = std::fs::read_dir(args.path).unwrap();
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
