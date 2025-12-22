use crate::{
    EvalError, ExecutionContext, RcI,
    data_types::{
        WfBoolean, WfData, WfDataType, WfFunction, WfFunctionCall, WfFunctionCallInner, WfTypedList,
    },
};

pub fn first_element(list: WfTypedList, context: &ExecutionContext) -> Result<WfData, EvalError> {
    list.split_first_element(Some(context))
        .map(|(x, _)| x)
        .map_err(|(e, _)| e)
}

pub fn list_equality(
    list1: WfTypedList,
    list2: WfTypedList,
    equality_function: WfFunction,
    context: &ExecutionContext,
) -> Result<WfData, EvalError> {
    if list1.len() != list2.len() {
        return Ok(WfBoolean::new(false).into_wf_data());
    };
    for (ele1, ele2) in list1.iter_checked(context).zip(list2.iter_checked(context)) {
        // they have been evaluated already as part as the iter_checked
        //TODO: still evaluate them in case this change in the future
        let ele1 = ele1?;
        let ele2 = ele2?;

        let function_call = WfFunctionCall(RcI::new(WfFunctionCallInner {
            function: equality_function.clone(),
            args: vec![ele1, ele2],
        }));
        let evaluated = function_call.evaluate(context).map_err(|(e, _)| e)?;
        let as_boolean = WfBoolean::parse(evaluated, context).map_err(|(e, _)| e)?;
        if !as_boolean.value {
            return Ok(WfBoolean::new(false).into_wf_data());
        }
    }

    Ok(WfBoolean::new(true).into_wf_data())
}
