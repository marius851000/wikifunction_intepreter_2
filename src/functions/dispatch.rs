use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, Zid,
    data_types::{
        WfBoolean, WfData, WfDataType, WfFunction, WfFunctionCall, WfString, WfTypedList,
    },
    eval_error::TraceEntry,
    functions::{boolean, list, logic, string},
    util::MaybeVec,
};

fn assert_args_count(expected_size: usize, list: &Vec<WfData>) -> Result<(), EvalError> {
    if expected_size == list.len() {
        return Ok(());
    } else {
        return Err(EvalError::from_kind(EvalErrorKind::TooManyArguments(
            list.len(),
            expected_size,
        )));
    }
}

///NOTE: branches are free to not evaluate their generated output (as long as it is a progression toward the result)
pub fn dispatch_builtins(
    function_zid: Zid,
    call: &WfFunctionCall,
    context: &ExecutionContext,
) -> Result<(WfData, bool, MaybeVec<TraceEntry>), EvalError> {
    //TODO: proper error tracing.
    //TODO: only evaluate necessary input (for when some are discarded, such as the if function)
    let mut args_evaluated = Vec::new();
    for (pos, arg) in call.0.args.iter().enumerate() {
        args_evaluated.push(arg.clone().evaluate(context).map_err(|(e, _)| {
            e.inside_key(KeyIndex::from_u32s_panic(
                Some(function_zid.0.get()),
                Some(pos as u32 + 1),
            ))
        })?);
    }

    match function_zid.0.get() {
        802 => {
            assert_args_count(3, &args_evaluated)?;
            let r#else = args_evaluated.pop().unwrap();
            let r#then = args_evaluated.pop().unwrap();
            let boolean =
                WfBoolean::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            return Ok(logic::if_function(boolean, r#then, r#else));
        }
        811 => {
            assert_args_count(1, &args_evaluated)?;
            let list1 =
                WfTypedList::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            //is evaluation needed? no it isn’t.
            return list::first_element(list1, context);
        }
        844 => {
            assert_args_count(2, &args_evaluated)?;
            let bool2 = WfBoolean::parse(args_evaluated.pop().unwrap(), context)
                .map_err(|(e, _)| e.inside_key(keyindex!(844, 2)))?;
            let bool1 = WfBoolean::parse(args_evaluated.pop().unwrap(), context)
                .map_err(|(e, _)| e.inside_key(keyindex!(844, 1)))?;
            return Ok((
                boolean::boolean_equality(bool1, bool2).into_wf_data(),
                false,
                MaybeVec::default(),
            ));
        }
        866 => {
            assert_args_count(2, &args_evaluated)?;
            let string2 =
                WfString::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            let string1 =
                WfString::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            return Ok((
                string::string_equality(string1, string2).into_wf_data(),
                false,
                MaybeVec::default(),
            ));
        }
        889 => {
            assert_args_count(3, &args_evaluated)?;
            let equality_function = WfFunction::parse(args_evaluated.pop().unwrap(), context)
                .map_err(|(e, _)| e.inside_key(keyindex!(889, 3)))?;
            let list2 = WfTypedList::parse(args_evaluated.pop().unwrap(), context)
                .map_err(|(e, _)| e.inside_key(keyindex!(889, 2)))?;
            let list1 = WfTypedList::parse(args_evaluated.pop().unwrap(), context)
                .map_err(|(e, _)| e.inside_key(keyindex!(889, 1)))?;
            return list::list_equality(list1, list2, equality_function, context)
                .map(|v| (v.into_wf_data(), true, MaybeVec::default()));
        }
        _ => return Err(EvalError::from_kind(EvalErrorKind::NoBuiltin(function_zid))),
    }
}
