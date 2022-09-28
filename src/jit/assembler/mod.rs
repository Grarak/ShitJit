use std::fmt::Formatter;
use std::{fmt, result};

pub mod instructions_assembler;
pub mod registers_handler;
pub mod short_inst;

pub struct Error {
    err: String,
}

impl Error {
    pub fn new(err: String) -> Self {
        Error { err }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.err.fmt(f)
    }
}
