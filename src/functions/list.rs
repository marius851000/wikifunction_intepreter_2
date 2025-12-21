use crate::{
    EvalError,
    data_types::{WfData, WfTypedList},
};

pub fn first_element(list: WfTypedList) -> Result<WfData, EvalError> {
    list.split_first_element(true)
        .map(|(x, _)| x)
        .map_err(|(e, _)| e)
}
