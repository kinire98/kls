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
    #[arg(short='a')]
    all: bool,
    /// Do not list implied . and ..
    #[arg(short='A')]
    almost_all: bool,
    /// Author of the file (might come as an ID)
    #[arg(long)]
    author: bool, 
    /// Ignore files ending with ~
    #[arg(short='b')]
    ignore_backups: bool,
    /// Lists only directories
    #[arg(short='d')]
    directories: bool,
    /// Long listing
    #[arg(short='l')]
    long_listing: bool,
    /// Lists subdirectories recursively (can crash in case of file structure being too deep)
    #[arg(short='r')]
    recursive: bool, 
    /// Prints the size of the file
    #[arg(short='s')]
    size: bool,
}
impl Arguments {
    fn to_tuple(&self) -> (Option<PathBuf>, bool, bool, bool, bool, bool, bool, bool, bool) {
        (self.path.clone(), self.all, self.almost_all, self.author, self.ignore_backups, self.directories, self.long_listing, self.recursive, self.size)
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
    let mut tree = kls::Dir::from(args.to_tuple());
    tree.print()?;
    Ok(())
}
