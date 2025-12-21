use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI,
    data_types::{
        WfBoolean, WfData, WfDataType, WfFunction, WfFunctionCall, util::SubstitutionInfo,
        wf_function_call::FunctionCallOrType,
    },
};

#[derive(Clone, PartialEq, Debug)]
pub struct WfTestCaseInner {
    function: WfFunction,
    call: WfData,
    validation: WfData,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WfTestCase(pub RcI<WfTestCaseInner>);

impl WfTestCase {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfTestCase(test_case) = data {
            return Ok(test_case);
        }

        match data.check_z1k1(zid!(20), context) {
            Ok(_) => (),
            Err(e) => return Err((e, data)),
        };

        let function_unparsed =
            match get_value_from_data_err_handled!(data, keyindex!(20, 1)).evaluate(context) {
                Ok(v) => v,
                Err((e, _)) => return Err((e.inside_key(keyindex!(20, 1)), data)),
            };

        let function = match WfFunction::parse(function_unparsed, context) {
            Ok(f) => f,
            Err((e, _)) => return Err((e.inside_key(keyindex!(20, 1)), data)),
        };

        let call = get_value_from_data_err_handled!(data, keyindex!(20, 2));
        let validation = get_value_from_data_err_handled!(data, keyindex!(20, 3));

        Ok(Self(RcI::new(WfTestCaseInner {
            function,
            call,
            validation,
        })))
    }

    pub fn run_test(self, context: &ExecutionContext) -> Result<(), EvalError> {
        let test_result = match self.0.call.clone().evaluate(context) {
            Ok(v) => v,
            Err((e, _)) => return Err(e),
        };

        let patched_function_call = match WfFunctionCall::parse_for_test(
            self.0.validation.clone(),
            context,
            test_result.clone(),
        ) {
            Ok(FunctionCallOrType::FunctionCall(c)) => c,
            Ok(FunctionCallOrType::Type(_type)) => {
                return Err(EvalError::from_kind(
                    EvalErrorKind::ExpectedFunctionCallGotType,
                ));
            } //TODO: trace
            Err((e, _)) => return Err(e),
        };

        let boolean_result_unparsed = match patched_function_call.evaluate(context) {
            Ok(result) => result,
            Err((e, _)) => return Err(e), //TODO: trace
        };

        let boolean_result = match WfBoolean::parse(boolean_result_unparsed, context) {
            Ok(result) => result,
            Err((e, _)) => return Err(e), //TODO: trace
        };

        if boolean_result.value {
            Ok(())
        } else {
            Err(EvalError::from_kind(
                EvalErrorKind::TestCaseFailedWithFalse(Box::new(test_result)),
            ))
        }
    }
}

impl WfDataType for WfTestCase {
    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(20)))
        } else if key == keyindex!(20, 1) {
            Some(self.0.function.clone().into_wf_data())
        } else if key == keyindex!(20, 2) {
            Some(self.0.call.clone())
        } else if key == keyindex!(20, 3) {
            Some(self.0.validation.clone())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![
            keyindex!(1, 1),
            keyindex!(20, 1),
            keyindex!(20, 2),
            keyindex!(20, 3),
        ]
    }

    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfTestCase(self)
    }

    fn evaluate(self, _context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        todo!("evaluate the function (but not the call and validator)");
    }

    fn substitute_function_arguments<I: SubstitutionInfo>(
        self,
        info: &I,
        context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        Ok(Self(RcI::new(WfTestCaseInner {
            function: match WfFunction::parse(
                self.0
                    .function
                    .clone()
                    .substitute_function_arguments(info, context)
                    .map_err(|e| e.inside_key(keyindex!(20, 1)))?,
                context,
            ) {
                Ok(v) => v,
                Err((e, _)) => return Err(e.inside_key(keyindex!(20, 2))),
            },
            call: self
                .0
                .call
                .clone()
                .substitute_function_arguments(info, context)
                .map_err(|e| e.inside_key(keyindex!(20, 2)))?,
            validation: self
                .0
                .validation
                .clone()
                .substitute_function_arguments(info, context)
                .map_err(|e| e.inside_key(keyindex!(20, 3)))?,
        }))
        .into_wf_data())
    }
}
