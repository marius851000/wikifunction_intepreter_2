use crate::data_types::{WfBoolean, WfData};

pub fn if_function(boolean: WfBoolean, r#if: WfData, r#else: WfData) -> WfData {
    if boolean.value { r#if } else { r#else }
}
