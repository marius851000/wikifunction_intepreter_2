#[macro_use]
mod zid;
pub use zid::Zid;

pub mod data_types;

mod eval_error;
pub use eval_error::{EvalError, EvalErrorKind};

mod execution_context;
pub use execution_context::ExecutionContext;

pub mod util;

mod global_context;
pub use global_context::GlobalContext;
