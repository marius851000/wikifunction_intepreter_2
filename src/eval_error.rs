use std::{error::Error, fmt::Display};

use thiserror::Error;

use crate::Zid;

#[derive(Error, Debug)]
pub enum EvalErrorKind {
    #[error("Parsing zid: {0}")]
    ParseZid(#[source] anyhow::Error),
    #[error("Missing key: {0}")]
    MissingKey(Zid),
    #[error("Expected reference")]
    NotAReference,
    #[error("Wrong type, got {0}, expected {1}")]
    WrongType(Zid, Zid),
    #[error("Incorrect identity reference for boolean {0}")]
    IncorrectIdentityForBoolean(Zid),
    #[error("Persistent object {0} does not exist")]
    MissingPersistentObject(Zid),
}

#[derive(Debug, Clone)]
#[allow(dead_code)] //TODO:
pub enum TraceEntry {
    Inside(Zid),
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

    pub fn missing_key(key: Zid) -> Self {
        Self::from_kind(EvalErrorKind::MissingKey(key))
    }

    pub fn trace_str(mut self, text: &str) -> Self {
        self.trace.push(TraceEntry::Text(text.to_string()));
        self
    }

    pub fn inside(mut self, zid: Zid) -> Self {
        self.trace.push(TraceEntry::Inside(zid));
        self
    }
}

impl Error for EvalError {
    fn cause(&self) -> Option<&dyn Error> {
        return Some(&self.kind);
    }
}

//TODO: a proper error display
impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(std::fmt::Debug::fmt(&self, f)?)
    }
}
