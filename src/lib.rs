/*

Docs for attributes constants for files
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