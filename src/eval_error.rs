use std::{error::Error, fmt::Display};

use thiserror::Error;

use crate::{KeyIndex, KeyIndexParseError, Zid, data_types::WfData};

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EvalErrorKind {
    #[error("Parsing zid: {0}")]
    ParseKeyIndex(#[source] KeyIndexParseError),
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
    #[error("Expected to find a type an identity key")]
    NoIdentity,
    #[error(
        "There is way, way too many argument in this function. So much it overflow the counter."
    )]
    TooManyArgsInFunction,
    #[error("A type’s type should be either Z4 or indirectly Z7")]
    WrongTypeZidForType,
    #[error("A Z7 type turned out to apparently not be a type disguised as function")]
    ExpectedTypeGotFunction,
    #[error("A Z14 implementation expect only one kind of implementation")]
    ExpectOnlyOneImplementation,
    #[error("A Z14 should contain one implementation, but none were found")]
    ExpectOneImplementionFoundZero,
    #[error("No implementation for function {0}")]
    /// traces point to the function
    NoImplementationForFunction(Zid),
    #[error("{0} arguments provided, expected {1}")]
    TooManyArguments(usize, usize),
    #[error("Argument reference reference {0}, which does not have the needed K part")]
    ArgumentReferenceNoKPart(KeyIndex),
    #[error("No argument found at 0-indexed position {0}")]
    ArgumentReferenceTooLarge(u32),
    #[error("Built-in for function {0} is not implemented or inexistant")]
    NoBuiltin(Zid),
    #[error("Expected function call, found type")]
    ExpectedFunctionCallGotType,
    #[error("Test case failed with \"false\" result. Intermediate result: {0:?}")]
    TestCaseFailedWithFalse(Box<WfData>),
    #[error("Can’t get head of an empty list")]
    CantGetHeadOfEmptyList,
    #[error("type does not match")]
    TypeDoesNotMatch,
    #[error("unimplemented: {0}")]
    Unimplemented(String),
    #[error("recursed too deep, aborting due to risk of stack overflow")]
    RecursedTooDeep,
    #[error("This explictly invalid data shouldn’t be reached outside of unit test")]
    TestData,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] //TODO:
pub enum TraceEntry {
    InsideKey(KeyIndex),
    InsideList(usize), // position is starting from 0 for the first element of the list (which exclude the argument paramater)
    InsideReference(Zid),
    //TODO: find a better way to identity implementation
    /// This should only be used once all the argument had been parsed and substituted (is that kind of stuff even necessary?)
    DuringSubstitution(Zid), // zid is the ZID of the function
    Substituted(Zid),
    ProcessingNonCompositionFunction(Zid),
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

    pub fn unimplemented(text: String) -> Self {
        Self::from_kind(EvalErrorKind::Unimplemented(text))
    }

    pub fn get_kind(&self) -> &EvalErrorKind {
        &self.kind
    }

    pub fn get_trace(&self) -> &Vec<TraceEntry> {
        return &self.trace;
    }

    pub fn trace(mut self, of: TraceEntry) -> Self {
        self.trace.push(of);
        self
    }

    pub fn trace_str(self, text: &str) -> Self {
        self.trace(TraceEntry::Text(text.to_string()))
    }

    pub fn inside_key(self, key: KeyIndex) -> Self {
        self.trace(TraceEntry::InsideKey(key))
    }

    pub fn inside_list(self, pos: usize) -> Self {
        self.trace(TraceEntry::InsideList(pos))
    }

    pub fn inside_reference_to(self, zid: Zid) -> Self {
        self.trace(TraceEntry::InsideReference(zid))
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
