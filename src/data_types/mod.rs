#[macro_use]
mod util;

#[macro_use]
mod wf_data_type;
pub use wf_data_type::WfDataType;

mod wf_data;
pub use wf_data::WfData;

mod wf_boolean;
pub use wf_boolean::WfBoolean;

mod wf_reference;
pub use wf_reference::WfReference;

mod wf_string;
pub use wf_string::WfString;

mod wf_untyped;
pub use wf_untyped::WfUntyped;

mod wf_invalid;
pub use wf_invalid::WfInvalid;

mod wf_typed_list;
pub use wf_typed_list::WfTypedList;

mod wf_function;
pub use wf_function::{WfFunction, WfFunctionInner};

mod wf_function_call;
pub use wf_function_call::WfFunctionCall;

mod wf_implementation;
pub use wf_implementation::{ImplementationByKind, WfImplementation, WfImplementationInner};

mod wf_argument_reference;
pub use wf_argument_reference::WfArgumentReference;

mod wf_test_case;
pub use wf_test_case::{WfTestCase, WfTestCaseInner};

pub mod types_def;

#[derive(Debug, PartialEq, Clone)]
pub enum MaybeEvaluated<T: std::fmt::Debug + PartialEq + Clone> {
    Unchecked(WfData),
    Valid(T),
}
