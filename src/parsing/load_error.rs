use sonic_rs::Value;
use thiserror::Error;

use crate::{KeyIndex, KeyIndexParseError, ZidParseError};

#[derive(Error, Debug, PartialEq)]
pub enum LoadError {
    #[error("Couldn’t parse Key index {0}")]
    CantParseKeyIndex(String, #[source] KeyIndexParseError),
    #[error("Can’t parse ZID")]
    CantParseZID(String, #[source] ZidParseError),
    #[error("inside object entry {0}")]
    InsideMap(KeyIndex, Box<LoadError>),
    #[error("inside array entry {0}")]
    InsideArray(usize, Box<LoadError>),
    #[error("unsupported data type in {0}")]
    InvalidDataType(Value),
    #[error("Extra field {0:?} in a String ZObject")]
    ExtraFieldInString(String),
    #[error(
        "First character of a string that looks like reference is not Z. They should be wrapped into a {{\"Z1K1\": \"Z6\", \"Z6K1\": <string>}} (in string {0:?})"
    )]
    UpperCaseFirstCharOutsideZ6(String),
    #[error(
        "Empty array found. The first element of an array is its type (including Z1 for untyped list)"
    )]
    EmptyArray,
}
