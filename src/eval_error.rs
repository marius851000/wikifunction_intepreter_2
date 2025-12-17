use std::{error::Error, fmt::Display};

use thiserror::Error;

use crate::{KeyIndex, KeyIndexParseError, Zid};

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvalErrorKind {
    #[error("Parsing zid: {0}")]
    ParseZid(#[source] KeyIndexParseError),
    #[error("Missing key: {0}")]
    MissingKey(KeyIndex),
    #[error("Expected reference")]
    NotAReference,
    #[error("Wrong ZID type, got {0}, expected {1}")]
    WrongType(Zid, Zid),
    #[error("Incorrect identity reference for boolean {0}")]
    IncorrectIdentityForBoolean(Zid),
    #[error("Persistent object {0} does not exist")]
    MissingPersistentObject(Zid),
    #[error("Not a standard type that can be expressed as just a ZID")]
    NotStandardType,
    #[error("This explictly invalid data shouldn’t be reached outside of unit test")]
    TestData,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] //TODO:
pub enum TraceEntry {
    Inside(KeyIndex),
    Text(String),
}

#[derive(Debug)]
pub struct EvalError {
    kind: EvalErrorKind,
    trace: Vec<TraceEntry>, //TODO: a more refined type later
}

impl EvalError {
    pub fn from_kind(kind: EvalErrorKind) -> Self {
        Self {
            kind,
            trace: Vec::new(),
        }
    }

    pub fn missing_key(key: KeyIndex) -> Self {
        Self::from_kind(EvalErrorKind::MissingKey(key))
    }

    pub fn trace_str(mut self, text: &str) -> Self {
        self.trace.push(TraceEntry::Text(text.to_string()));
        self
    }

    pub fn inside(mut self, zid: KeyIndex) -> Self {
        self.trace.push(TraceEntry::Inside(zid));
        self
    }
}

impl Error for EvalError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(&self.kind)
    }
}

//TODO: a proper error display
impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
