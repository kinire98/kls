use std::path::PathBuf;
use clap::*;
use std::env::current_dir;
use kls::error::Result;



#[derive(Parser, Debug)]
/// If conflicting options it will apply the minimal one
pub struct Arguments {
    /// Specified path, if empty the default would be the current dir
    path: Option<PathBuf>,
    /// Do not ignore entries starting with .
    #[arg(short='a', long)]
    all: bool,
    /// Do not list implied . and ..
    #[arg(short='A', long)]
    almost_all: bool,
    /// Ignore files ending with ~
    #[arg(short='b', long)]
    ignore_backups: bool,
    /// Lists only directories
    #[arg(short='d', long)]
    directories: bool,
    /// Long listing
    #[arg(short='l')]
    long_listing: bool,
    /// Prints the size of the file (in bytes)
    #[arg(short='s', long)]
    size: bool,
    /// Lists subdirectories recursively (max 255)
    #[arg(short='r', long, default_value="0",)]
    recursive: u8, 
}
impl Arguments {
    fn to_tuple(&self) -> (Option<PathBuf>, bool, bool, bool, bool, bool, bool, u8) {
        (self.path.clone(), self.all, self.almost_all, self.ignore_backups, self.directories, self.long_listing, self.size, self.recursive)
    }
}
/*
? Flags to implement
  -a, --all                  do not ignore entries starting with .
  -A, --almost-all           do not list implied . and ..
      --author               with -l, print the author of each file
  -B, --ignore-backups       do not list implied entries ending with ~
  -d, --directory            list directories themselves, not their contents
  -l                         use a long listing format
  -R, --recursive            list subdirectories recursively
  -s, --size                 print the allocated size of each file, in blocks
*/


fn main() -> Result<()> {
    let mut args = Arguments::parse();
    if args.path.is_none() {
        args.path = Some(current_dir().unwrap());
    }
    let mut dir = kls::Dir::from(args.to_tuple());
    dir.print()?;
    Ok(())
}
