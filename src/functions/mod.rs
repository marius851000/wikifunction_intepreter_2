//! All the functon here, expect the dispatch function, assume the arguments have already be evaluated.

pub mod boolean;
pub mod list;
pub mod logic;
pub mod string;

mod dispatch;
pub use dispatch::dispatch_builtins;
