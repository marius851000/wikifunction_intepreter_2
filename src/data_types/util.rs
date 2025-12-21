use crate::{EvalError, data_types::WfData};

#[macro_export]
macro_rules! get_value_from_data_err_handled {
    ($data:expr, $key:expr) => {
        match $data.get_key_err($key) {
            Ok(v) => v,
            Err(e) => return Err((e, $data)),
        }
    };
}

pub trait SubstitutionInfo {
    /// using 0-indexing
    fn get_for_pos(&self, pos: u32) -> Result<WfData, EvalError>;
}
