use std::collections::BTreeMap;

use sonic_rs::{Array, JsonValueTrait, Object, Value, ValueRef};

use crate::{
    KeyIndex, RcI, Zid,
    data_types::{
        WfData, WfDataType, WfReference, WfString, WfUncheckedTypedList, WfUncheckedTypedListInner,
        WfUntyped,
    },
    parsing::LoadError,
};

pub fn parse_value(source: &Value) -> Result<WfData, LoadError> {
    match source.as_ref() {
        ValueRef::Object(object) => Ok(parse_object(object)?),
        ValueRef::String(text) => Ok(parse_str(text)?),
        ValueRef::Array(array) => Ok(parse_array(array)?),
        _ => Err(LoadError::InvalidDataType(source.clone())),
    }
}

pub fn parse_object(source: &Object) -> Result<WfData, LoadError> {
    // special case: explicitly typed string
    let mut have_string_type = false;
    let mut z6k1_is_str = false;
    for (k, v) in source.iter() {
        have_string_type |= k == "Z1K1" && v.as_str() == Some("Z6");
        z6k1_is_str |= k == "Z6K1" && v.as_str().is_some();
    }
    if have_string_type && z6k1_is_str {
        for (k, _v) in source.iter() {
            if k != "Z1K1" && k != "Z6K1" {
                return Err(LoadError::ExtraFieldInString(k.to_string()));
            }
        }
        let text = source
            .get(&"Z6K1")
            .expect("have_string_type is true, Z6K1 should be present")
            .as_str()
            .expect("z6k1_is_str is true, str expect");
        return Ok(WfString::new(text).into_wf_data());
    }
    // standard situation
    let mut temp_map = BTreeMap::new();
    for (k, v) in source.iter() {
        let zid =
            KeyIndex::from_str(k).map_err(|e| LoadError::CantParseKeyIndex(k.to_string(), e))?;
        let inner_object = parse_value(v).map_err(|e| LoadError::InsideMap(zid, Box::new(e)))?;
        temp_map.insert(zid, inner_object);
    }
    Ok(WfUntyped::new(temp_map).into_wf_data())
}

/// Depending on the formatting, may return either a reference or a string.
/// String that looks like a reference are handling in parse_object
pub fn parse_str(source: &str) -> Result<WfData, LoadError> {
    match source.chars().next() {
        None => Ok(WfString::new(source).into_wf_data()),
        Some(first_char) => {
            if first_char.is_ascii_uppercase() {
                match first_char {
                    'Z' => {
                        let zid = Zid::from_str(source)
                            .map_err(|e| LoadError::CantParseZID(source.to_string(), e))?;
                        Ok(WfReference::new(zid).into_wf_data())
                    }
                    _ => Err(LoadError::UpperCaseFirstCharOutsideZ6(source.to_string())),
                }
            } else {
                Ok(WfString::new(source).into_wf_data())
            }
        }
    }
}

pub fn parse_array(array: &Array) -> Result<WfData, LoadError> {
    let mut iterator = array.iter();
    let type_value = iterator.next().ok_or(LoadError::EmptyArray)?;
    let r#type = parse_value(type_value).map_err(|e| LoadError::InsideArray(0, Box::new(e)))?;

    let mut entries = Vec::with_capacity(array.len());
    for (loop_count, value) in iterator.enumerate() {
        entries.push(
            parse_value(value).map_err(|e| LoadError::InsideArray(loop_count + 1, Box::new(e)))?,
        );
    }

    Ok(
        WfUncheckedTypedList(RcI::new(WfUncheckedTypedListInner { r#type, entries }))
            .into_wf_data(),
    )
}

#[cfg(test)]
mod tests {
    use sonic_rs::{Array, Object, from_str};

    use crate::{
        RcI,
        data_types::{
            WfData, WfDataType, WfReference, WfString, WfUncheckedTypedList,
            WfUncheckedTypedListInner,
        },
        parsing::parse_json::{parse_array, parse_object, parse_str},
    };

    #[test]
    fn test_load_strings() {
        assert_eq!(
            parse_object(&from_str::<Object>(r#"{"Z1K1": "Z6", "Z6K1": "Z4"}"#).unwrap()).unwrap(),
            WfString::new("Z4").into_wf_data()
        );
        assert_eq!(parse_str("p4").unwrap(), WfString::new("p4").into_wf_data());
        assert_eq!(parse_str("").unwrap(), WfString::new("").into_wf_data());
    }

    #[test]
    fn test_load_reference() {
        assert_eq!(
            parse_str("Z4").unwrap(),
            WfReference::new(zid!(4)).into_wf_data()
        );

        parse_str("Z4K1").unwrap_err();
        parse_str("K4").unwrap_err();
        parse_str("P4").unwrap_err();
    }

    #[test]
    fn test_array() {
        assert_eq!(
            parse_array(&from_str::<Array>(r#"["Z6", "hello", "world"]"#).unwrap()).unwrap(),
            WfData::WfUncheckedTypedList(WfUncheckedTypedList(RcI::new(
                WfUncheckedTypedListInner {
                    r#type: WfData::new_reference(zid!(6)),
                    entries: vec![
                        WfString::new("hello").into_wf_data(),
                        WfString::new("world").into_wf_data()
                    ]
                }
            )))
        );

        parse_array(&from_str::<Array>(r#"[]"#).unwrap()).unwrap_err();
    }
}
