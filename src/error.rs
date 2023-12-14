use std::error;
use std::fmt::{Display, Debug};
use colored::Colorize;

pub type Result<T> = std::result::Result<T, self::Error>;


pub struct Error {
    pub kind: ErrorKind,
    pub problem: String
}
pub enum ErrorKind {
    DirNotFound,
    ConflictingFlag,
    OsError,
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::DirNotFound => write!(f, "The directory you specified ({}) doesn't exist.", self.problem.bold().white().on_black()),
            ErrorKind::ConflictingFlag => write!(f, "The flags you specified ({}) can't be put together.", self.problem.bold().white().on_black()),
            ErrorKind::OsError => write!(f, "An error with the os has ocurred: {}", self.problem.bold().white().on_black())
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::DirNotFound => write!(f, "The directory you specified ({}) doesn't exist.", self.problem.bold().white().on_black()),
            ErrorKind::ConflictingFlag => write!(f, "The flags you specified ({}) can't be put together.", self.problem.bold().white().on_black()),
            ErrorKind::OsError => write!(f, "An error with the os has ocurred: {}", self.problem.bold().white().on_black())
        }
    }
}
impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}