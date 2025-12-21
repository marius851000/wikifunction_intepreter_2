use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI,
    data_types::{WfData, WfDataType},
};
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationByKind {
    Composition(WfData),
    Code(WfData),
    Builtin(WfData),
}

impl ImplementationByKind {
    pub fn get_key_index(&self) -> KeyIndex {
        match self {
            ImplementationByKind::Composition(_) => keyindex!(14, 2),
            ImplementationByKind::Code(_) => keyindex!(14, 3),
            ImplementationByKind::Builtin(_) => keyindex!(14, 4),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WfImplementationInner {
    pub function: WfData,
    // assume one implementation by Z14
    pub r#impl: ImplementationByKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WfImplementation(pub RcI<WfImplementationInner>);

impl WfImplementation {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfImplementation(i) = data {
            return Ok(i);
        };

        match data.check_z1k1(zid!(14), context) {
            Ok(_) => (),
            Err(e) => return Err((e, data)),
        };

        let function = get_value_from_data_err_handled!(data, keyindex!(14, 1));

        let composition = data.get_key(keyindex!(14, 2));
        let code = data.get_key(keyindex!(14, 3));
        let builtin = data.get_key(keyindex!(14, 4));
        if ((if composition.is_some() { 1 } else { 0 })
            + (if code.is_some() { 1 } else { 0 })
            + (if builtin.is_some() { 1 } else { 0 }))
            >= 2
        {
            return Err((
                EvalError::from_kind(EvalErrorKind::ExpectOnlyOneImplementation),
                data,
            ));
        }

        let r#impl = if let Some(composition) = composition {
            ImplementationByKind::Composition(composition)
        } else if let Some(code) = code {
            ImplementationByKind::Code(code)
        } else if let Some(builtin) = builtin {
            ImplementationByKind::Builtin(builtin)
        } else {
            return Err((
                EvalError::from_kind(EvalErrorKind::ExpectOneImplementionFoundZero),
                data,
            ));
        };

        Ok(Self(RcI::new(WfImplementationInner { function, r#impl })))
    }
}

impl WfDataType for WfImplementation {
    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(14)))
        } else if key == keyindex!(14, 1) {
            Some(self.0.function.clone())
        } else if key == keyindex!(14, 2) {
            if let ImplementationByKind::Composition(x) = &self.0.r#impl {
                Some(x.clone())
            } else {
                None
            }
        } else if key == keyindex!(14, 3) {
            if let ImplementationByKind::Code(x) = &self.0.r#impl {
                Some(x.clone())
            } else {
                None
            }
        } else if key == keyindex!(14, 4) {
            if let ImplementationByKind::Builtin(x) = &self.0.r#impl {
                Some(x.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![
            keyindex!(1, 1),
            keyindex!(14, 1),
            self.0.r#impl.get_key_index(),
        ]
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfImplementation(self)
    }
}
