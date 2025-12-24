use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI,
    data_types::{
        WfBoolean, WfData, WfDataType, WfFunction, WfFunctionCall, util::SubstitutionInfo,
        wf_function_call::FunctionCallOrType,
    },
    eval_error::TraceEntry,
};

#[derive(Clone, PartialEq, Debug)]
pub struct WfTestCaseInner {
    pub function: WfFunction,
    pub call: WfData,
    pub validation: WfData,
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

    pub fn get_validation_function_call_with_patched_first_input(
        self,
        first_input: WfData,
        context: &ExecutionContext,
    ) -> Result<WfFunctionCall, EvalError> {
        match WfFunctionCall::parse_for_test(self.0.validation.clone(), context, first_input) {
            Ok(FunctionCallOrType::FunctionCall(c)) => Ok(c),
            Ok(FunctionCallOrType::Type(_type)) => {
                return Err(EvalError::from_kind(
                    EvalErrorKind::ExpectedFunctionCallGotType,
                ));
            } //TODO: trace
            Err((e, _)) => return Err(e),
        }
    }

    pub fn run_test(self, context: &ExecutionContext) -> Result<(), EvalError> {
        let test_result = match self.0.call.clone().evaluate(context) {
            Ok(v) => v,
            Err((e, _)) => return Err(e.inside_key(keyindex!(20, 2))),
        };

        let patched_function_call = self
            .get_validation_function_call_with_patched_first_input(test_result.clone(), context)?;

        let boolean_result_unparsed = match patched_function_call.evaluate(context) {
            Ok(result) => result,
            Err((e, _)) => {
                return Err(e.trace(TraceEntry::CheckingTestCaseResult(test_result.clone())));
            }
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
