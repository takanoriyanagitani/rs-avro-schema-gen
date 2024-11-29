use std::io;

use serde_json::Map;
use serde_json::Value;

#[derive(serde::Serialize)]
pub struct FlatField {
    pub name: String,
    pub r#type: String,
}

#[derive(serde::Serialize)]
pub struct FlatRecord {
    pub name: String,
    pub r#type: String,
    pub namespace: Option<String>,
    pub fields: Vec<FlatField>,
}

pub fn json_sample_data2schema(
    name: String,
    namespace: Option<String>,
    j: Value,
) -> Result<FlatRecord, io::Error> {
    let m: Map<String, Value> = match j {
        Value::Object(o) => Ok(o),
        _ => Err(io::Error::other(format!("invalid input json: {j}"))),
    }?;
    let pairs = m.into_iter();
    let rfields = pairs.map(|pair| {
        let (key, val) = pair;
        let typname: Result<&str, io::Error> = match val {
            Value::Null => Err(io::Error::other("null value got")),
            Value::Bool(_) => Ok("boolean"),
            Value::Number(n) => match n.is_f64() {
                true => Ok("double"),
                false => Ok("long"),
            },
            Value::String(_) => Ok("string"),
            Value::Array(_) => Err(io::Error::other("array got")),
            Value::Object(_) => Err(io::Error::other("object got")),
        };
        typname.map(|name: &str| FlatField {
            name: key,
            r#type: name.into(),
        })
    });
    let collected: Result<Vec<_>, _> = rfields.collect();
    let fields: Vec<FlatField> = collected?;
    Ok(FlatRecord {
        name,
        r#type: "record".into(),
        namespace,
        fields,
    })
}

pub fn json_sample_string2schema(
    name: String,
    namespace: Option<String>,
    jstr: String,
) -> Result<FlatRecord, io::Error> {
    let val: Value = serde_json::from_str(jstr.as_str()).map_err(io::Error::other)?;
    json_sample_data2schema(name, namespace, val)
}

pub fn json_sample_bytes2schema(
    name: String,
    namespace: Option<String>,
    jbytes: Vec<u8>,
) -> Result<FlatRecord, io::Error> {
    let val: Value = serde_json::from_slice(&jbytes).map_err(io::Error::other)?;
    json_sample_data2schema(name, namespace, val)
}
