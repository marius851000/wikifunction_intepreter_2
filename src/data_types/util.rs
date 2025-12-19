#[macro_export]
macro_rules! get_value_from_data_err_handled {
    ($data:expr, $key:expr) => {
        match $data.get_key_err($key) {
            Ok(v) => v,
            Err(e) => return Err((e, $data)),
        }
    };
}
