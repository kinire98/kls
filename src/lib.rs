/*

? Docs for attributes constants for files in Windows
File attribute constant: 
https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/file-attribute-constants


Const attributes:
1 -> readonly
2 -> hidden
4 -> system file
16 -> is dir
32 -> file
128 -> normal xD


It uses binary bits for each attribute
for example if is a hidden dir it would be:
is dir -> 16 + hidden -> 2 = 18 -> hidden dir
128 cannot be mixed



For linux there is a crate file_mode:
https://docs.rs/file-mode/latest/file_mode/
https://crates.io/crates/file-mode
Saves me
*/



/*
? To investigate properties
https://doc.rust-lang.org/std/fs/struct.Metadata.html
https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html#tymethod.file_attributes

*/

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