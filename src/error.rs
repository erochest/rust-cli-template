use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
