use error::Error;
use serde_json::Value;
use std::collections::BTreeMap;

pub fn decode_array<T, F: Fn(Value) -> Result<T, Error>>(
    value: Value,
    f: F,
) -> Result<Vec<T>, Error> {
    match value {
        Value::Array(arr) => arr.into_iter().map(f).collect(),
        _ => Err(Error::Decode("Error decoding object", value)),
    }
}

pub fn into_map(value: Value) -> Result<BTreeMap<String, Value>, Error> {
    match value {
        Value::Object(m) => Ok(m),
        value => Err(Error::Decode("Expected object", value)),
    }
}

pub fn remove(map: &mut BTreeMap<String, Value>, key: &str) -> Result<Value, Error> {
    map.remove(key)
        .ok_or_else(|| Error::Decode("Unexpected absent key", Value::String(key.into())))
}
