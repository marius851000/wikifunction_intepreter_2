use std::sync::Arc;

use crate::GlobalContext;

pub struct ExecutionContext {
    global_context: Arc<GlobalContext>,
}

impl ExecutionContext {
    pub fn default_for_global(global_context: Arc<GlobalContext>) -> Self {
        Self { global_context }
    }

    pub fn get_global(&self) -> &GlobalContext {
        &self.global_context
    }
}
