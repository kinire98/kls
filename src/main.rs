use std::io;
use std::fs;
use std::os::windows::prelude::*;






fn main() {
    let thing = std::fs::read_dir(r".\").unwrap();
    for path in thing {
        let jeje = path.unwrap().path().display().to_string();
        let metadata = fs::metadata(jeje.clone()).unwrap();
        let atts = metadata.file_attributes();
        let file_size = metadata.file_size();
        println!("{}: {}, {}",jeje,  atts, file_size);
    }
}
