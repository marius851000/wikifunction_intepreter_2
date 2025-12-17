use std::fmt::Debug;

use crate::{EvalError, EvalErrorKind, ExecutionContext, KeyIndex, data_types::WfData};

pub trait WfDataType: Debug + Clone {
    fn into_wf_data(self) -> WfData;
    /// used to know that this structure is one of the final type. Used to know that inequality mean two object with this property does not represent the same thing.
    fn is_fully_realised(&self) -> bool;
    fn get_identity_key(&self) -> Option<KeyIndex>;
    /// does not evaluate
    fn get_key(&self, key: KeyIndex) -> Option<WfData>;
    /// does not evaluate
    fn list_keys(&self) -> Vec<KeyIndex>; //TODO: some iterator?

    /// Follow references and all that -- recursively. Default to returning self.
    /// Also need to guarantee the data is correct. It shouldn’t return a WfUntyped.
    fn evaluate(self, _context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        Ok(self.into_wf_data())
    }

    fn get_reference(self, _context: &ExecutionContext) -> Result<KeyIndex, (EvalError, Self)> {
        Err((EvalError::from_kind(EvalErrorKind::NotAReference), self))
    }
}

/// Macro that generates the `WfDataType` implementation for a generic enum.
///
/// * **$Struct** – The enum type that implements `WfDataType`
/// * **$into_wf_data_expr** – Expression that produces a `WfData` from the enum
/// * **$variant ($inner)** – One or more enum variants and the name of the
///   inner field that implements `WfDataType`.
///
/// Example usage:
/// ```
/// use interpreter2::impl_wf_data_type;
/// use interpreter2::data_types::{WfBoolean, WfString};
///
/// #[derive(Debug, Clone)]
/// enum WfBooleanOrString {
///     WfBoolean(WfBoolean),
///     WfString(WfString),
/// }
///
/// impl_wf_data_type!(
///     WfBooleanOrString,
///     |this: WfBooleanOrString| this.into_wf_data(),
///     WfBoolean(d),
///     WfString(d)
/// );
/// ```
///
/// Thanks gpt-oss:20b (with barely one mistake. And doctest that did not ran. Not that I suceeded too. Until clippy pointed out $crate)
#[macro_export]
macro_rules! impl_wf_data_type {
    ($Struct:ident, $into_wf_data_expr:expr, $( $variant:ident ($inner:ident) ),+ ) => {
        impl $crate::data_types::WfDataType for $Struct {
            fn into_wf_data(self) -> $crate::data_types::WfData {
                $into_wf_data_expr(self)
            }

            fn is_fully_realised(&self) -> bool {
                match self {
                    $(Self::$variant($inner) => $inner.is_fully_realised(),)+
                }
            }

            fn get_identity_key(&self) -> Option<$crate::KeyIndex> {
                match self {
                    $(Self::$variant($inner) => $inner.get_identity_key(),)+
                }
            }

            fn get_key(&self, key: $crate::KeyIndex) -> Option<$crate::data_types::WfData> {
                match self {
                    $(Self::$variant($inner) => $inner.get_key(key),)+
                }
            }

            fn list_keys(&self) -> Vec<$crate::KeyIndex> {
                match self {
                    $(Self::$variant($inner) => $inner.list_keys(),)+
                }
            }

            fn evaluate(self, context: &$crate::ExecutionContext) -> Result<$crate::data_types::WfData, ($crate::EvalError, Self)> {
                match self {
                    $(Self::$variant($inner) =>
                        $inner.evaluate(context).map_err(|(e, p)| (e, Self::$variant(p))),)+
                }
            }

            fn get_reference(self, context: &$crate::ExecutionContext) -> Result<$crate::KeyIndex, ($crate::EvalError, Self)> {
                match self {
                    $(Self::$variant($inner) =>
                        $inner.get_reference(context).map_err(|(e, p)| (e, Self::$variant(p))),)+
                }
            }
        }
    }
}
