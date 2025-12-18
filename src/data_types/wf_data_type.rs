use std::fmt::Debug;

use crate::{EvalError, EvalErrorKind, ExecutionContext, KeyIndex, Zid, data_types::WfData};

pub trait WfDataType: Debug + Clone {
    fn into_wf_data(self) -> WfData;
    /// used to know that this structure is one of the final type. Used to know that inequality mean two object with this property does not represent the same thing.
    fn is_fully_realised(&self) -> bool;
    /// Return the key that store the identity as a reference to itself. Note that it expect data to be either a direct WfReference to self, or such a dereferenced reference (which shouldn’t usually occur).
    fn get_identity_zid_key(&self) -> Option<KeyIndex>;
    /// does not evaluate
    fn get_key(&self, key: KeyIndex) -> Option<WfData>;
    /// does not evaluate
    fn list_keys(&self) -> Vec<KeyIndex>; //TODO: some iterator?

    /// Follow references and all that -- recursively. Default to returning self.
    /// Also need to guarantee the returned data is correct and valid on the first level (but deeper data need to themselve be .evaluate-d). It shouldn’t return a WfUntyped.
    /// It is allowed to return an error if a child is unvalid (still, not a requirement)
    fn evaluate(self, _context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        Ok(self.into_wf_data())
    }

    /// This only return this reference, not recursive reference
    fn get_reference(
        self,
        _context: &ExecutionContext,
    ) -> Result<(Zid, WfData), (EvalError, WfData)> {
        Err((
            EvalError::from_kind(EvalErrorKind::NotAReference),
            self.into_wf_data(),
        ))
    }

    /// Like get_key, but if the key is missing, it mark the error as having the key missing, ready to be returned if not inside another key itself (maybe once the owned WfData is added to it)
    fn get_key_err(&self, key: KeyIndex) -> Result<WfData, EvalError> {
        if let Some(data) = self.get_key(key) {
            Ok(data)
        } else {
            Err(EvalError::missing_key(key))
        }
    }

    /// Like get_identity_zid, but return an error if the found identity key does not match expected value, and with early return if self is a reference to the identity
    fn check_identity_zid(
        self,
        context: &ExecutionContext,
        expected_value: Zid,
    ) -> Result<WfData, (EvalError, WfData)> {
        // fast path
        let self2 = match self.get_reference(context) {
            Ok((reference, consumed)) => {
                if reference == expected_value {
                    return Ok(consumed.into_wf_data());
                } else {
                    consumed
                }
            }
            Err((_, consumed)) => consumed,
        };

        // slow path
        let evaluated = match self2.evaluate(context) {
            Ok(v) => v,
            Err((e, n)) => return Err((e, n.into_wf_data())),
        };

        let identity_key = match evaluated.get_identity_zid_key() {
            Some(k) => k,
            _ => return Err((EvalError::from_kind(EvalErrorKind::NoIdentity), evaluated)),
        };

        let gotten = match evaluated.get_identity_zid(context, identity_key) {
            Ok(zid) => zid,
            Err(e) => return Err((e, evaluated)),
        };

        if gotten != expected_value {
            Err((
                EvalError::from_kind(EvalErrorKind::WrongType(gotten, expected_value)),
                evaluated,
            ))
        } else {
            Ok(evaluated)
        }
    }

    /// Default implementation makes use of self.get_identity_zid_key and recursive calls
    /// The only reason to implement this manually if for performance. In the fast path, it should only clone a reference, if not optimised out.
    /// The reason is ask for a key is in case of using it over WfUntyped, as it is often used inside parsing function
    fn get_identity_zid(
        &self,
        context: &ExecutionContext,
        identity_key: KeyIndex,
    ) -> Result<Zid, EvalError> {
        let identity_value = match self.get_key_err(identity_key) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if let WfData::WfReference(reference) = identity_value {
            return Ok(reference.to);
        }

        let evaluated = match identity_value.get_reference(context) {
            Ok(k) => return Ok(k.0),
            Err((_, evaluated)) => evaluated,
        };

        match evaluated.get_identity_zid(context, identity_key) {
            Ok(k) => return Ok(k),
            Err(e) => Err(e.inside(identity_key)),
        }
    }

    fn should_be_evaluated_before_parsing(&self) -> bool {
        return false;
    }

    /// To be called in .parse if it attempt to parse a reference or something like that should be .evaluated beforehand.
    /// Sanity check. May be turned into a debug_panic eventually.
    fn assert_evaluated(&self) {
        if self.should_be_evaluated_before_parsing() {
            panic!("internal error: should have been evaluated: {:?}", self)
        }
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

            fn get_identity_zid_key(&self) -> Option<$crate::KeyIndex> {
                match self {
                    $(Self::$variant($inner) => $inner.get_identity_zid_key(),)+
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

            fn get_reference(self, context: &$crate::ExecutionContext) -> Result<($crate::Zid, $crate::data_types::WfData), ($crate::EvalError, $crate::data_types::WfData)> {
                match self {
                    $(Self::$variant($inner) =>
                        $inner.get_reference(context).map(|(z, p)| (z, p)).map_err(|(e, p)| (e, p)),)+
                }
            }
        }
    }
}
