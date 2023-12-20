/*
? Flags to implement
? Implemented
  -a, --all                  do not ignore entries starting with .
? Implmented
  -A, --almost-all           do not list implied . and ..
? Implemented
  -B, --ignore-backups       do not list implied entries ending with ~ 
? Implemented
  -d, --directory            list directories themselves, not their contents
  -l                         use a long listing format
? Implemented
  -r, --recursive            list subdirectories recursively
? Implemented
  -s, --size                 print the allocated size of each file, in blocks
*/
#![recursion_limit = "256"]





pub mod error;
use error::*;
mod sys;
use sys::*;

use std::path::PathBuf;
use colored::Colorize;



pub struct Dir {
    path: PathBuf,
    all: bool,
    almost_all: bool, 
    ignore_backups: bool,
    directories: bool,
    long_listing: bool,
    size: bool,
    recursive: u8,
}
impl Dir {
    pub fn from(args: (Option<PathBuf>, bool, bool, bool, bool, bool, bool, u8)) -> Self {
        Dir {
            path: args.0.unwrap(),
            all: args.1,
            almost_all: args.2,
            ignore_backups: args.3,
            directories: args.4,
            long_listing: args.5,
            size: args.6,
            recursive: args.7
        }
    }
    pub fn print(&mut self) -> Result<()> {
        if self.recursive > 0 {
            self.print_recursive(self.path.clone(), self.recursive)?;
        } else {
            self.print_dir(self.path.clone())?;
        }
        Ok(())
    }
    fn print_recursive(&mut self, path: PathBuf, recursion_times: u8) -> Result<()> {
        if path.is_file() {
            return Ok(());
        }
        self.print_dir(path.clone())?;
        if recursion_times == 0 {
            return Ok(());
        }
        std::fs::read_dir(path).unwrap().for_each(|child_path| {
            self.print_recursive(child_path.unwrap().path(), recursion_times - 1).unwrap();
        });
        Ok(())
    }
    fn print_dir(&self, parent_path: PathBuf) -> Result<()> {
        let mut parent_path_string = parent_path.display().to_string();
        if parent_path_string.ends_with('/') || parent_path_string.ends_with('\\') {
            parent_path_string.remove(parent_path_string.len() - 1);
        }
        if self.recursive > 0 {
            println!("{}", parent_path_string.on_truecolor(100, 255, 100).truecolor(50, 50, 255).bold());
        }
        if self.all {
            self.print_file(".".into())?;
            self.print_file("..".into())?;
        }
        std::fs::read_dir(parent_path.clone()).unwrap().for_each(|child_path| {
            let path = match child_path {
                Ok(path) => path.path(),
                Err(_) => todo!(),
            };
            let length_of_parent_path = parent_path_string.len() + 1;
            let mut to_print = path.display().to_string();
            if to_print.ends_with('~') && self.ignore_backups {
                return;
            }
            if to_print.chars().nth(length_of_parent_path).unwrap_or(' ') == '.' && !self.all && !self.almost_all {
                return;
            }
            to_print = to_print[length_of_parent_path..].to_string();
            let props = if self.long_listing {
                self.long_listing(&path)
            } else {
                "".to_string()
            };
            let size = if self.size && !self.long_listing {
                format!("{} ", self.size(&path))
            } else {
                "".to_string()
            };
            if path.is_file() {
                if !self.directories {
                    print!("{}{} {} {}", props, size, to_print.green(), if self.long_listing { "\n" } else { "" })
                }
            } else {
                print!("{}{} {} {}", props, size, to_print.on_truecolor(100, 255, 100).truecolor(50, 50, 255).bold(), if self.long_listing { "\n" } else { "" })
            }
        });
        println!();
        Ok(())
    }
    fn print_file(&self, path: PathBuf) -> Result<()> {
        let props = if self.long_listing {
            self.long_listing(&path)
        } else {
            "".to_string()
        };
        let size = if self.size && !self.long_listing {
            format!("{} ", self.size(&path))
        } else {
            "".to_string()
        };
        print!("{}{} {} {}", props, size, path.display().to_string().on_truecolor(100, 255, 100).truecolor(50, 50, 255).bold(), if self.long_listing {"\n"} else {""});
        Ok(())
    }
    fn long_listing(&self, path: &PathBuf) -> String {
        let size = self.size(path);
        #[cfg(windows)]
        let props = windows::MetadataPath::from(path.clone()).props();
        #[cfg(unix)]
        let props = unix::MetadataPath::from(path.clone()).props();
        format!("{} {}", props, size)
    }
    fn size(&self, path: &PathBuf) -> u64 {
        #[cfg(windows)]
        {
            windows::MetadataPath::from(path.clone()).size()
        }
        #[cfg(unix)]
        {
            unix::MetadataPath::from(path.clone()).size()
        }
    }
    
}


