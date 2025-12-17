#![feature(test)]

#[macro_use]
mod key_index;
use std::rc::Rc;

pub use key_index::{KeyIndex, KeyIndexParseError};

pub mod data_types;

mod eval_error;
pub use eval_error::{EvalError, EvalErrorKind};

mod execution_context;
pub use execution_context::ExecutionContext;

pub mod util;

pub mod parsing;

mod global_context;
pub use global_context::GlobalContext;

#[cfg(test)]
mod bench;

type RcI<T> = Rc<T>;
