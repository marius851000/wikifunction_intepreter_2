use crate::{
    EvalError, ExecutionContext, RcI,
    data_types::{
        WfBoolean, WfData, WfDataType, WfFunction, WfFunctionCall, WfFunctionCallInner, WfTypedList,
    },
    eval_error::TraceEntry,
    util::MaybeVec,
};

pub fn first_element(
    list: WfTypedList,
    context: &ExecutionContext,
) -> Result<(WfData, bool, MaybeVec<TraceEntry>), EvalError> {
    list.split_first_element(Some(context))
        .map(|(x, _)| (x, true, MaybeVec::One(TraceEntry::InsideList(0))))
        .map_err(|(e, _)| e)
}

pub fn list_equality(
    list1: WfTypedList,
    list2: WfTypedList,
    equality_function: WfFunction,
    context: &ExecutionContext,
) -> Result<WfBoolean, EvalError> {
    if list1.len() != list2.len() {
        return Ok(WfBoolean::new(false));
    };
    for (pos, (ele1, ele2)) in list1
        .iter_checked(context)
        .zip(list2.iter_checked(context))
        .enumerate()
    {
        // they have been evaluated already as part as the iter_checked
        //TODO: still evaluate them in case this change in the future
        let ele1 = ele1.map_err(|e| e.inside_list(pos).inside_key(keyindex!(889, 1)))?;
        let ele2 = ele2.map_err(|e| e.inside_list(pos).inside_key(keyindex!(889, 2)))?;

        let function_call = WfFunctionCall(RcI::new(WfFunctionCallInner {
            function: equality_function.clone(),
            args: vec![ele1, ele2],
        }));

        //TODO: trace
        let evaluated = function_call.clone().evaluate(context).map_err(|(e, _)| {
            e.trace(TraceEntry::ProcessingReconstructedData(
                function_call.into_wf_data(),
            ))
        })?;
        let as_boolean = WfBoolean::parse(evaluated, context).map_err(|(e, _)| todo!())?;
        if !as_boolean.value {
            return Ok(WfBoolean::new(false));
        }
    }

    Ok(WfBoolean::new(true))
}
