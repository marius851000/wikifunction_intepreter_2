use crate::{
    data_types::{WfBoolean, WfData},
    eval_error::TraceEntry,
    util::MaybeVec,
};

pub fn if_function(
    boolean: WfBoolean,
    then: WfData,
    r#else: WfData,
) -> (WfData, bool, MaybeVec<TraceEntry>) {
    if boolean.value {
        (
            then,
            true,
            MaybeVec::One(TraceEntry::InsideKey(keyindex!(802, 2))),
        )
    } else {
        (
            r#else,
            true,
            MaybeVec::One(TraceEntry::InsideKey(keyindex!(802, 3))),
        )
    }
}
