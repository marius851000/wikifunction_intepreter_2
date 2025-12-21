use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex,
    data_types::{WfData, WfDataType, WfString},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfArgumentReference {
    pub key_id: KeyIndex,
}

impl WfArgumentReference {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfArgumentReference(ar) = data {
            return Ok(ar);
        }

        match data.check_z1k1(zid!(18), context) {
            Ok(_) => (),
            Err(e) => return Err((e, data)),
        };

        let key_index_evaluated =
            match get_value_from_data_err_handled!(data, keyindex!(18, 1)).evaluate(context) {
                Ok(k) => k,
                Err((e, _)) => return Err((e.inside_key(keyindex!(18, 1)), data)),
            };

        let key_index_as_string = match WfString::parse(key_index_evaluated, context) {
            Ok(k) => k,
            Err((e, _)) => return Err((e.inside_key(keyindex!(18, 1)), data)),
        };

        let key_index = match KeyIndex::from_str(&key_index_as_string.text) {
            Ok(k) => k,
            Err(e) => {
                return Err((
                    EvalError::from_kind(EvalErrorKind::ParseKeyIndex(e))
                        .inside_key(keyindex!(18, 1)),
                    data,
                ));
            }
        };

        Ok(Self { key_id: key_index })
    }
}

impl WfDataType for WfArgumentReference {
    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![keyindex!(1, 1), keyindex!(18, 1)]
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(14)))
        } else if key == keyindex!(18, 1) {
            Some(WfString::new(&self.key_id.to_string()).into_wf_data())
        } else {
            None
        }
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfArgumentReference(self)
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn should_be_evaluated_before_parsing(&self) -> bool {
        true
    }

    fn substitute_function_arguments<I: super::util::SubstitutionInfo>(
        self,
        info: &I,
        _context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        let k_part = if let Some(v) = self.key_id.get_k() {
            v
        } else {
            return Err(EvalError::from_kind(
                EvalErrorKind::ArgumentReferenceNoKPart(self.key_id),
            ));
        };
        let index = k_part.get() - 1;
        info.get_for_pos(index)
    }
}

#[cfg(test)]
mod tests {
    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext, RcI,
        data_types::{WfArgumentReference, WfData, WfDataType, WfString},
    };

    #[test]
    fn test_parse() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));

        let unparsed_data = WfData::from_map(btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(18)),
            keyindex!(18, 1) => WfString::new("Z881K1").into_wf_data(),
        });

        assert_eq!(
            WfArgumentReference::parse(unparsed_data, &context).unwrap(),
            WfArgumentReference {
                key_id: keyindex!(881, 1)
            }
        )
    }
}
