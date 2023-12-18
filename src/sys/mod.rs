
/*
? To investigate properties
https://doc.rust-lang.org/std/fs/struct.Metadata.html
https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.file_attributes

*/


#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
pub mod windows;
// 
#[cfg(windows)]
pub use windows::*;