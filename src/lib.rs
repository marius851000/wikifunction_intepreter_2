#![feature(test)]

#[macro_use]
mod zid;
use std::rc::Rc;

pub use zid::{Zid, ZidParseError};

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
