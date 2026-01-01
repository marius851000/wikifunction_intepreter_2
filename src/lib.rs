#![feature(test)]

#[macro_use]
mod key_index;
pub use key_index::{KeyIndex, KeyIndexParseError};

#[macro_use]
mod zid;
pub use zid::{Zid, ZidParseError};

pub mod data_types;

mod eval_error;
pub use eval_error::{EvalError, EvalErrorKind, TraceEntry};

mod execution_context;
pub use execution_context::ExecutionContext;

pub mod util;

pub mod parsing;

mod global_context;
pub use global_context::GlobalContext;

pub mod functions;
pub mod replay;

#[cfg(test)]
mod bench;

use std::rc::Rc;
/// Global lock to be used for everything in this interpreter that is stored inside of GlobalContext.
/// May be (eventually optionally) switched to an Arc in the future for multi-threading.
pub type RcI<T> = Rc<T>;
