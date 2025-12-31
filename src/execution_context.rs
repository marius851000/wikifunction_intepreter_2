use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{EvalError, EvalErrorKind, GlobalContext, RcI};

pub struct ExecutionContext {
    global_context: RcI<GlobalContext>,
    function_call_depth: AtomicUsize,
    function_call_count: AtomicUsize,
}

impl ExecutionContext {
    pub fn default_for_global(global_context: RcI<GlobalContext>) -> Self {
        Self {
            global_context,
            function_call_depth: AtomicUsize::new(0),
            function_call_count: AtomicUsize::new(0),
        }
    }

    pub fn get_global(&self) -> &GlobalContext {
        &self.global_context
    }

    pub fn check_can_run_function_and_acquire_guard<'l>(
        &'l self,
    ) -> Result<FunctionCallDepthGuard<'l>, EvalError> {
        if self.function_call_depth.fetch_add(1, Ordering::Relaxed) > 100 {
            self.function_call_depth.fetch_sub(1, Ordering::Relaxed);
            return Err(EvalError::from_kind(EvalErrorKind::RecursedTooDeep));
        }
        if self.function_call_count.fetch_add(1, Ordering::Relaxed) > 100_000 {
            return Err(EvalError::from_kind(
                EvalErrorKind::FunctionCallCountExceeded,
            ));
        }

        //TODO: also just do a timeout. Probably won’t need a function call count then.

        return Ok(FunctionCallDepthGuard {
            value: &self.function_call_depth,
        });
    }
}

pub struct FunctionCallDepthGuard<'l> {
    value: &'l AtomicUsize,
}

impl<'l> Drop for FunctionCallDepthGuard<'l> {
    fn drop(&mut self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }
}
