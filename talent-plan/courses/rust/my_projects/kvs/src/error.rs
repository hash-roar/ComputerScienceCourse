use failure::Fail;
use std::{io, string::FromUtf8Error};

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
    #[fail(display = "sled error {}", _0)]
    SledErr(#[cause] sled::Error),
    #[fail(display = "error happen convrt from u8 {}", _0)]
    FromU8Err(#[cause] FromUtf8Error),
    #[fail(display = "error: {}", _0)]
    StrErr(String),
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

impl From<sled::Error> for DbError {
    fn from(err: sled::Error) -> Self {
        DbError::SledErr(err)
    }
}

impl From<FromUtf8Error> for DbError {
    fn from(err: FromUtf8Error) -> Self {
        DbError::FromU8Err(err)
    }
}

pub type Result<T> = std::result::Result<T, DbError>;
