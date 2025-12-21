//! All the functon here, expect the dispatch function, assume the arguments have already be evaluated.

pub mod boolean;

mod dispatch;
pub use dispatch::dispatch_builtins;
