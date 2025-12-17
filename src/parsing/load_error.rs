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
    #[error("Extra field {0:?} in a String ZObject")]
    ExtraFieldInString(String),
    #[error(
        "First character of a string that looks like reference is not Z. They should be wrapped into a {{\"Z1K1\": \"Z6\", \"Z6K1\": <string>}} (in string {0:?})"
    )]
    UpperCaseFirstCharOutsideZ6(String),
}
