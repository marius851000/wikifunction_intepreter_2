use crate::data_types::WfBoolean;

pub fn boolean_equality(arg1: WfBoolean, arg2: WfBoolean) -> WfBoolean {
    WfBoolean::new(arg1.value == arg2.value)
}
