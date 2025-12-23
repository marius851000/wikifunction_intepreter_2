use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{EvalError, EvalErrorKind, GlobalContext, RcI};

pub struct ExecutionContext {
    global_context: RcI<GlobalContext>,
    function_call_count: AtomicUsize,
}

impl ExecutionContext {
    pub fn default_for_global(global_context: RcI<GlobalContext>) -> Self {
        Self {
            global_context,
            function_call_count: AtomicUsize::new(0),
        }
    }

    pub fn get_global(&self) -> &GlobalContext {
        &self.global_context
    }

    pub fn check_can_run_function_and_acquire_guard<'l>(
        &'l self,
    ) -> Result<FunctionCallCountGuard<'l>, EvalError> {
        if self.function_call_count.fetch_add(1, Ordering::Relaxed) > 100 {
            self.function_call_count.fetch_sub(1, Ordering::Relaxed);
            return Err(EvalError::from_kind(EvalErrorKind::RecursedTooDeep));
        } else {
            return Ok(FunctionCallCountGuard {
                value: &self.function_call_count,
            });
        }
    }
}

pub struct FunctionCallCountGuard<'l> {
    value: &'l AtomicUsize,
}

impl<'l> Drop for FunctionCallCountGuard<'l> {
    fn drop(&mut self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }
}
