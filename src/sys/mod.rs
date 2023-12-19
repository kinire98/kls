
/*
? To investigate properties
https://doc.rust-lang.org/std/fs/struct.Metadata.html

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