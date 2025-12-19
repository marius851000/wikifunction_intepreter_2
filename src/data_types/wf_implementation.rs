use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfImplementationInner {
    function: WfData,
    composition: WfData,
    code: WfData,
    builtin: WfData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WfImplementation(RcI<WfImplementationInner>);

impl WfImplementation {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfImplementation(i) = data {
            return Ok(i);
        };

        match data.check_z1k1(zid!(14), context) {
            Ok(_) => (),
            Err(e) => return Err((e, data)),
        };

        let function = match data.get_key_err(keyindex!(14, 1)) {
            Ok(v) => v,
            Err(e) => return Err((e, data)),
        };

        let composition = match data.get_key_err(keyindex!(14, 2)) {
            Ok(v) => v,
            Err(e) => return Err((e, data)),
        };

        let code = match data.get_key_err(keyindex!(14, 3)) {
            Ok(v) => v,
            Err(e) => return Err((e, data)),
        };

        let builtin = match data.get_key_err(keyindex!(14, 4)) {
            Ok(v) => v,
            Err(e) => return Err((e, data)),
        };

        Ok(Self(RcI::new(WfImplementationInner {
            function,
            composition,
            code,
            builtin,
        })))
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
            Some(self.0.composition.clone())
        } else if key == keyindex!(14, 3) {
            Some(self.0.code.clone())
        } else if key == keyindex!(14, 4) {
            Some(self.0.builtin.clone())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![
            keyindex!(1, 1),
            keyindex!(14, 1),
            keyindex!(14, 2),
            keyindex!(14, 3),
            keyindex!(14, 4),
        ]
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfImplementation(self)
    }
}
