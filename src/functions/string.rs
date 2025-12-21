use crate::data_types::{WfBoolean, WfString};

pub fn string_equality(arg1: WfString, arg2: WfString) -> WfBoolean {
    WfBoolean::new(arg1.text == arg2.text)
}
