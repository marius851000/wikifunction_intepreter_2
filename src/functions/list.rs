use crate::{
    EvalError, ExecutionContext,
    data_types::{WfData, WfTypedList},
};

pub fn first_element(list: WfTypedList, context: &ExecutionContext) -> Result<WfData, EvalError> {
    list.split_first_element(true, context)
        .map(|(x, _)| x)
        .map_err(|(e, _)| e)
}
