use crate::{GlobalContext, RcI};

pub struct ExecutionContext {
    global_context: RcI<GlobalContext>,
}

impl ExecutionContext {
    pub fn default_for_global(global_context: RcI<GlobalContext>) -> Self {
        Self { global_context }
    }

    pub fn get_global(&self) -> &GlobalContext {
        &self.global_context
    }
}
