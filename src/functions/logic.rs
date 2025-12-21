use crate::data_types::{WfBoolean, WfData};

pub fn if_function(boolean: WfBoolean, then: WfData, r#else: WfData) -> WfData {
    if boolean.value { r#then } else { r#else }
}
