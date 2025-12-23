use crate::{
    EvalError, ExecutionContext,
    data_types::{WfBoolean, WfData, WfDataType},
};

pub fn if_function(
    boolean: WfBoolean,
    then: WfData,
    r#else: WfData,
    context: &ExecutionContext,
) -> Result<WfData, EvalError> {
    if boolean.value {
        then.evaluate(context)
            .map_err(|(e, _)| e.inside_key(keyindex!(802, 2)))
    } else {
        r#else
            .evaluate(context)
            .map_err(|(e, _)| e.inside_key(keyindex!(802, 3)))
    }
}
