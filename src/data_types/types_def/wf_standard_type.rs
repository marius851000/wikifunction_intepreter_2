use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI, Zid,
    data_types::{WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, PartialEq, Clone)]
pub struct WfStandardTypeInner {
    pub identity_ref: Zid,
    pub keys: WfData,
    pub validator: WfData,
    pub equality: Option<WfData>,
    pub display_function: Option<WfData>,
    pub reading_function: Option<WfData>,
    pub type_converters_to_code: Option<WfData>,
    pub type_converters_from_code: Option<WfData>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WfStandardType {
    pub inner: RcI<WfStandardTypeInner>,
}

impl From<WfStandardTypeInner> for WfStandardType {
    fn from(value: WfStandardTypeInner) -> Self {
        Self {
            inner: RcI::new(value),
        }
    }
}

impl WfStandardType {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfType(WfTypeGeneric::WfStandardType(t)) = data {
            return Ok(t);
        }

        match data.check_z1k1(zid!(4), context) {
            Ok(_) => (),
            Err(e) => return Err((e, data)),
        };

        let identity_ref = match data.get_identity_zid(context, keyindex!(4, 1)) {
            Ok(k) => k,
            Err(e) => return Err((e, data)),
        };

        let keys = get_value_from_data_err_handled!(data, keyindex!(4, 2));
        let validator = get_value_from_data_err_handled!(data, keyindex!(4, 3));
        let equality = data.get_key(keyindex!(4, 4));
        let display_function = data.get_key(keyindex!(4, 5));
        let reading_function = data.get_key(keyindex!(4, 6));
        let type_converters_to_code = data.get_key(keyindex!(4, 7));
        let type_converters_from_code = data.get_key(keyindex!(4, 8));

        Ok(Self {
            inner: RcI::new(WfStandardTypeInner {
                identity_ref,
                keys,
                validator,
                equality,
                display_function,
                reading_function,
                type_converters_to_code,
                type_converters_from_code,
            }),
        })
    }
}

impl WfDataType for WfStandardType {
    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        Some(keyindex!(4, 1))
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(4)))
        } else if key == keyindex!(4, 1) {
            Some(WfData::new_reference(self.inner.identity_ref))
        } else if key == keyindex!(4, 2) {
            Some(self.inner.keys.clone())
        } else if key == keyindex!(4, 3) {
            Some(self.inner.validator.clone())
        } else if key == keyindex!(4, 4) {
            self.inner.equality.clone()
        } else if key == keyindex!(4, 5) {
            self.inner.display_function.clone()
        } else if key == keyindex!(4, 6) {
            self.inner.reading_function.clone()
        } else if key == keyindex!(4, 7) {
            self.inner.type_converters_to_code.clone()
        } else if key == keyindex!(4, 8) {
            self.inner.type_converters_from_code.clone()
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        let mut result = vec![
            keyindex!(1, 1),
            keyindex!(4, 1),
            keyindex!(4, 2),
            keyindex!(4, 3),
        ];
        if self.inner.equality.is_some() {
            result.push(keyindex!(4, 4));
        }
        if self.inner.display_function.is_some() {
            result.push(keyindex!(4, 5));
        }
        if self.inner.reading_function.is_some() {
            result.push(keyindex!(4, 6));
        }
        if self.inner.type_converters_to_code.is_some() {
            result.push(keyindex!(4, 7));
        }
        if self.inner.type_converters_from_code.is_some() {
            result.push(keyindex!(4, 8));
        }
        result
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfStandardType(self))
    }

    fn is_fully_realised(&self) -> bool {
        true
    }
}
