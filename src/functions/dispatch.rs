use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
    data_types::{WfBoolean, WfData, WfDataType, WfFunctionCall, WfString},
    functions::{boolean, logic, string},
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
pub fn dispatch_builtins(
    function_zid: Zid,
    call: &WfFunctionCall,
    context: &ExecutionContext,
) -> Result<WfData, EvalError> {
    //TODO: proper error tracing.
    let mut args_evaluated = Vec::new();
    for arg in call.0.args.iter() {
        args_evaluated.push(arg.clone().evaluate(context).map_err(|(e, _)| e)?);
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
        844 => {
            assert_args_count(2, &args_evaluated)?;
            let bool2 =
                WfBoolean::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            let bool1 =
                WfBoolean::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            return Ok(boolean::boolean_equality(bool1, bool2).into_wf_data());
        }
        866 => {
            assert_args_count(2, &args_evaluated)?;
            let string2 =
                WfString::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            let string1 =
                WfString::parse(args_evaluated.pop().unwrap(), context).map_err(|(e, _)| e)?;
            return Ok(string::string_equality(string1, string2).into_wf_data());
        }
        _ => return Err(EvalError::from_kind(EvalErrorKind::NoBuiltin(function_zid))),
    }
}
