use std::result;

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, Zid,
    data_types::{
        FunctionCallOrType, ImplementationByKind, WfData, WfDataType, WfFunction, WfFunctionCall,
        WfReference, WfTestCase, WfTypedList,
    },
    eval_error::TraceEntry,
};

#[derive(Debug)]
pub enum FullTraceEntry {
    // WfData is the result present by the key
    InsideKey(KeyIndex, WfData),
    InsideList(usize, WfData),
    FollowReference(Zid, WfData),
    AfterCompositionSubstitution(Zid, WfData),
    // just a marker to help debugging.
    ProcessingNonCompositionFunction(Zid),
    // first WfData is the result, second is the generated function call
    CheckingTestCaseResult(WfData, WfData),
}

impl FullTraceEntry {
    pub fn get_action_text(&self) -> String {
        match self {
            Self::InsideKey(index, _) => format!("inside {}", index),
            Self::InsideList(pos, _) => format!("inside list, at 0-indexed position {}", pos),
            Self::FollowReference(zid, _) => format!("follow reference to {}", zid),
            Self::AfterCompositionSubstitution(function_zid, _) => {
                format!("after substitution for function {}", function_zid)
            }
            Self::ProcessingNonCompositionFunction(function_zid) => {
                format!(
                    "Inside non-composition implementation for function {}",
                    function_zid
                )
            }
            Self::CheckingTestCaseResult(result, _) => {
                format!("Checking result with validator (result is {:?})", result)
            }
        }
    }

    pub fn get_result(&self) -> Option<&WfData> {
        match self {
            Self::InsideKey(_, d) => Some(d),
            Self::InsideList(_, d) => Some(d),
            Self::FollowReference(_, d) => Some(d),
            Self::AfterCompositionSubstitution(_, d) => Some(d),
            Self::ProcessingNonCompositionFunction(_) => None,
            Self::CheckingTestCaseResult(_, d) => Some(d),
        }
    }
}

#[derive(Debug)]
pub struct ReplayResult {
    pub full_trace: Vec<FullTraceEntry>,
    pub root: WfData,
}

impl ReplayResult {
    pub fn pretty_trace(&self) -> String {
        let mut result = String::new();
        for entry in &self.full_trace {
            result.extend(
                format!("{} -> {:?}\n", entry.get_action_text(), entry.get_result()).chars(),
            );
        }
        result.extend(format!("root error data structure:\n{:?}\n", self.root).chars());
        result
    }
}

//TODO: think about whether this should panic on error.
//(normally, replay should always be accurate. It not being as such is a bug in the software)
pub fn generate_replay(
    input: WfData,
    context: &ExecutionContext,
    error: &EvalError,
) -> ReplayResult {
    let mut current = input;
    let mut full_trace = Vec::new();

    let mut iterator = error.get_trace().iter().rev();
    // will iterate from higher level to lower level
    while let Some(step) = iterator.next() {
        //println!("{:?}: {:?}", step, current);
        match step {
            TraceEntry::InsideKey(key) => {
                current = current.get_key(*key).unwrap();
                full_trace.push(FullTraceEntry::InsideKey(*key, current.clone()))
            }
            TraceEntry::InsideList(pos) => {
                let list = WfTypedList::parse(current, context).unwrap();
                current = list.iter().skip(*pos).next().unwrap();
                full_trace.push(FullTraceEntry::InsideList(*pos, current.clone()))
            }
            TraceEntry::InsideReference(target_value) => {
                let reference = WfReference::parse(current, context).unwrap();
                assert_eq!(reference.to, *target_value);
                current = context
                    .get_global()
                    .get_object_value(&target_value)
                    .unwrap();
                full_trace.push(FullTraceEntry::FollowReference(
                    *target_value,
                    current.clone(),
                ));
            }
            TraceEntry::ProcessingNonCompositionFunction(function_zid) => full_trace.push(
                FullTraceEntry::ProcessingNonCompositionFunction(*function_zid),
            ),
            TraceEntry::Substituted(function_zid) => {
                let function_call = match WfFunctionCall::parse(current, context).unwrap() {
                    FunctionCallOrType::FunctionCall(f) => f,
                    FunctionCallOrType::Type(_) => panic!(),
                };
                assert_eq!(*function_zid, function_call.0.function.0.identity);
                let implementation = function_call.pick_implementation(context).unwrap();
                let composition = match &implementation.0.r#impl {
                    ImplementationByKind::Composition(composition) => composition,
                    _ => panic!(),
                };
                let propagated = composition
                    .clone()
                    .substitute_function_arguments(&function_call, context)
                    .unwrap();
                current = propagated;
                full_trace.push(FullTraceEntry::AfterCompositionSubstitution(
                    *function_zid,
                    current.clone(),
                ));
            }
            TraceEntry::CheckingTestCaseResult(result) => {
                let test_case = WfTestCase::parse(current, context).unwrap();
                current = test_case
                    .get_validation_function_call_with_patched_first_input(result.clone(), context)
                    .unwrap()
                    .into_wf_data();
                full_trace.push(FullTraceEntry::CheckingTestCaseResult(
                    result.clone(),
                    current.clone(),
                ));
            }
            _ => todo!("replay for {:?}", step),
        }
    }

    //NOTE: this is just for debug. Might be turned off eventually.
    match error.get_kind() {
        EvalErrorKind::NoImplementationForFunction(function_zid) => {
            assert_eq!(
                WfFunction::parse(current.clone().evaluate(context).unwrap(), context)
                    .unwrap()
                    .get_preffered_implementation(context)
                    .unwrap_err()
                    .get_kind(),
                &EvalErrorKind::NoImplementationForFunction(*function_zid)
            );
        }
        _ => {
            let _ = current.clone().evaluate(context).unwrap_err();
        }
    };

    return ReplayResult {
        full_trace,
        root: current,
    };
}

#[cfg(test)]
mod tests {
    use map_macro::btree_map;

    use crate::{
        EvalErrorKind, ExecutionContext, GlobalContext, RcI,
        data_types::{WfBoolean, WfData, WfDataType},
        eval_error::TraceEntry,
        replay::generate_replay,
    };

    #[test]
    fn test_simple_replay() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));

        let function_map = btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(7)),
            keyindex!(7, 1) => WfData::new_reference(zid!(844)),
            // boolean equality function
            keyindex!(844, 1) => WfData::from_map(btree_map! {
                keyindex!(1, 1) => WfData::new_reference(zid!(7)),
                keyindex!(7, 1) => WfData::new_reference(zid!(844)),
                keyindex!(844, 1) => WfData::unvalid(EvalErrorKind::TestData),
                keyindex!(844, 2) => WfBoolean::new(true).into_wf_data(),
            }),
            keyindex!(844, 2) => WfBoolean::new(true).into_wf_data(),
        };

        let unparsed = WfData::from_map(function_map);
        let err = unparsed.clone().evaluate(&context).unwrap_err();
        assert_eq!(err.0.get_kind(), &EvalErrorKind::TestData);
        assert_eq!(
            err.0.get_trace(),
            &vec![
                TraceEntry::InsideKey(keyindex!(844, 1)),
                TraceEntry::ProcessingNonCompositionFunction(zid!(844)),
                TraceEntry::InsideKey(keyindex!(844, 1)),
                TraceEntry::ProcessingNonCompositionFunction(zid!(844)),
            ]
        );

        let replay_result = generate_replay(unparsed.clone(), &context, &err.0);
        assert_eq!(replay_result.root, WfData::unvalid(EvalErrorKind::TestData));
    }
}
