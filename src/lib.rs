/*
? Flags to implement
  -a, --all                  do not ignore entries starting with .
  -A, --almost-all           do not list implied . and ..
      --author               with -l, print the author of each file
? Implemented
  -B, --ignore-backups       do not list implied entries ending with ~ 
  -d, --directory            list directories themselves, not their contents
  -l                         use a long listing format
? Implemented
  -r, --recursive            list subdirectories recursively
  -s, --size                 print the allocated size of each file, in blocks
*/
#![recursion_limit = "1024"]






mod sys;
use sys::*;
use std::{path::{PathBuf, Path}, fs::ReadDir, fmt::Display};
use colored::Colorize;
pub mod error;
use error::*;



pub struct Dir {
    path: PathBuf,
    all: bool,
    almost_all: bool, 
    author: bool,
    ignore_backups: bool,
    directories: bool,
    long_listing: bool,
    recursive: bool,
    size: bool
}
impl Dir {
    pub fn from(args: (Option<PathBuf>, bool, bool, bool, bool, bool, bool, bool, bool)) -> Self {
        Dir {
            path: args.0.unwrap(),
            all: args.1,
            almost_all: args.2,
            author: args.3,
            ignore_backups: args.4,
            directories: args.5,
            long_listing: args.6,
            recursive: args.7,
            size: args.8
        }
    }
    pub fn print(&self) -> Result<()> {
        if self.recursive {
            self.print_recursive(self.path.clone())?;
        } else {
            self.print_dir(self.path.clone())?;
        }
        Ok(())
    }
    fn print_recursive(&self, path: PathBuf) -> Result<()> {
        if path.is_file() {
            return Ok(());
        }
        self.print_dir(path.clone())?;
        std::fs::read_dir(path).unwrap().into_iter().for_each(|child_path| {
            self.print_recursive(child_path.unwrap().path()).unwrap();
        });
        Ok(())
    }
    fn print_dir(&self, parent_path: PathBuf) -> Result<()> {
        println!("{}", parent_path.display().to_string().on_green().blue().bold());
        std::fs::read_dir(parent_path.clone()).unwrap().into_iter().for_each(|child_path| {
            let path = match child_path {
                Ok(path) => path.path(),
                Err(_) => todo!(),
            };
            let to_print = path.display().to_string();
            if to_print.chars().last().unwrap() == '~' && self.ignore_backups {
                return;
            }
            if self.long_listing {
                self.long_listing(parent_path.clone());
            }
            if path.is_file() {
                print!(".{} ", &to_print[parent_path.clone().display().to_string().len()..])
            } else {
                print!("{}{} ", ".".bold().black().on_white(), &to_print[parent_path.clone().display().to_string().len()..].bold().black().on_white())
            }
        });
        println!();
        Ok(())
    }
    fn long_listing(&self, path: PathBuf) {

    }
}


