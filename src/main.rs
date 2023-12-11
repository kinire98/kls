use std::io;
use std::fs;
#[cfg(windows)]
use std::os::windows::prelude::*;






fn main() {

    #[cfg(windows)]
    let thing = std::fs::read_dir(r".\").unwrap();
    #[cfg(unix)]
    let thing = std::fs::read_dir(r"./").unwrap();
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
            let file_size = metadata.file_type();
            println!("{}: {:?}, {:?}",jeje,  atts, file_size);
        }
    }
}
