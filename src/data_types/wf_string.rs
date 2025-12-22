use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfString {
    pub text: RcI<str>,
}

impl WfString {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }

    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfString(s) = data {
            return Ok(s);
        };
        data.assert_evaluated();
        match data.check_z1k1(zid!(6), context) {
            Ok(()) => (),
            Err(e) => return Err((e, data)),
        };
        let below_string = match data.get_key_err(keyindex!(6, 1)) {
            Err(e) => return Err((e.inside_key(keyindex!(6, 1)), data)),
            Ok(v) => match v.evaluate(context) {
                Err((e, _)) => return Err((e.inside_key(keyindex!(6, 1)), data)),
                Ok(v) => match WfString::parse(v, context) {
                    Ok(v) => v,
                    Err((e, _)) => return Err((e.inside_key(keyindex!(6, 1)), data)),
                },
            },
        };
        Ok(below_string)
    }
}

impl WfDataType for WfString {
    /// Return None. z6k1 is indeed an identity, but not identity that point to ZID.
    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(6)))
        } else if key == keyindex!(6, 1) {
            Some(self.clone().into_wf_data())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![keyindex!(1, 1), keyindex!(6, 1)]
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfString(self)
    }

    fn substitute_function_arguments<I: super::util::SubstitutionInfo>(
        self,
        _info: &I,
        _context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        Ok(self.into_wf_data())
    }
}
