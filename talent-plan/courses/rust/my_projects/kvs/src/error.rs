use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum DbError {
    #[fail(display = "{}", _0)]
    IoErr(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    SerdeErr(#[cause] serde_json::Error),
    #[fail(display = "key not found")]
    KeyNotFoundErr,
    #[fail(display = "unexpected command")]
    UnexpectCommandErr,
}

impl From<io::Error> for DbError {
    fn from(err: io::Error) -> Self {
        DbError::IoErr(err)
    }
}
impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::SerdeErr(err)
    }
}

pub type Result<T> = std::result::Result<T, DbError>;
