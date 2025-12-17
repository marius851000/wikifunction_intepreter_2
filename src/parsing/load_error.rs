use sonic_rs::Value;
use thiserror::Error;

use crate::{Zid, ZidParseError};

#[derive(Error, Debug, PartialEq)]
pub enum LoadError {
    #[error("Couldn’t parse ZID {0}")]
    CantParseZid(String, #[source] ZidParseError),
    #[error("inside object entry {0}")]
    InsideMap(Zid, Box<LoadError>),
    #[error("unsupported data type in {0}")]
    InvalidDataType(Value),
}
