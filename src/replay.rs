use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, Zid,
    data_types::{WfData, WfDataType, WfFunction, WfReference},
    eval_error::TraceEntry,
};

#[derive(Debug)]
pub enum FullTraceEntry {
    InsideKey(KeyIndex, WfData), // WfData is the result returned by the key
    FollowReference(Zid, WfData),
}

impl FullTraceEntry {
    pub fn get_action_text(&self) -> String {
        match self {
            Self::InsideKey(index, _) => format!("inside {}", index),
            Self::FollowReference(zid, _) => format!("follow reference to {}", zid),
        }
    }

    pub fn get_result(&self) -> &WfData {
        match self {
            Self::InsideKey(_, d) => d,
            Self::FollowReference(_, d) => d,
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
        match step {
            TraceEntry::InsideKey(key) => {
                current = current.get_key(*key).unwrap();
                full_trace.push(FullTraceEntry::InsideKey(*key, current.clone()))
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
            _ => todo!(),
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
                TraceEntry::InsideKey(keyindex!(844, 1))
            ]
        );

        let replay_result = generate_replay(unparsed.clone(), &context, &err.0);
        assert_eq!(replay_result.root, WfData::unvalid(EvalErrorKind::TestData));
    }
}
