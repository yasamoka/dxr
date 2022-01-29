//#![warn(missing_docs)]
//#![warn(missing_debug_implementations)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod ser_de;

pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "value")]
pub struct Value {
    #[serde(rename = "$value")]
    value: Type,
}

impl Value {
    pub fn new(value: Type) -> Value {
        Value { value }
    }

    pub fn i4(value: i32) -> Value {
        Value::new(Type::Integer(value))
    }

    pub fn int(value: i32) -> Value {
        Value::i4(value)
    }

    #[cfg(feature = "i8")]
    pub fn i8(value: i64) -> Value {
        Value::new(Type::Long(value))
    }

    pub fn boolean(value: bool) -> Value {
        Value::new(Type::Boolean(value))
    }

    pub fn string(value: String) -> Value {
        Value::new(Type::String(value))
    }

    pub fn double(value: f64) -> Value {
        Value::new(Type::Double(value))
    }

    pub fn datetime(value: DateTime<Utc>) -> Value {
        Value::new(Type::DateTime(value))
    }

    pub fn base64(value: Vec<u8>) -> Value {
        Value::new(Type::Base64(value))
    }

    pub fn structure(value: Struct) -> Value {
        Value::new(Type::Struct { members: value.members })
    }

    pub fn array(value: Array) -> Value {
        Value::new(Type::Array { data: value.data })
    }

    #[cfg(feature = "nil")]
    pub fn nil() -> Value {
        Value::new(Type::Nil)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Type {
    #[serde(rename = "i4", alias = "int")]
    Integer(#[serde(rename = "$value")] i32),
    #[cfg(feature = "i8")]
    #[serde(rename = "i8")]
    Long(#[serde(rename = "$value")] i64),
    #[serde(rename = "boolean", with = "crate::ser_de::boolean")]
    Boolean(#[serde(rename = "$value")] bool),
    #[serde(rename = "string")]
    String(#[serde(rename = "$value")] String),
    #[serde(rename = "double")]
    Double(#[serde(rename = "$value")] f64),
    #[serde(rename = "dateTime.iso8601", with = "crate::ser_de::datetime")]
    DateTime(#[serde(rename = "$value")] DateTime<Utc>),
    #[serde(rename = "base64", with = "crate::ser_de::base64")]
    Base64(#[serde(rename = "$value")] Vec<u8>),
    #[serde(rename = "struct")]
    Struct {
        #[serde(default, rename = "member")]
        members: Vec<Member>,
    },
    #[serde(rename = "array")]
    Array {
        #[serde(default)]
        data: ArrayData,
    },
    #[cfg(feature = "nil")]
    #[serde(rename = "nil")]
    Nil,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "struct")]
pub struct Struct {
    #[serde(default, rename = "member")]
    members: Vec<Member>,
}

impl Struct {
    pub fn from_members(members: Vec<Member>) -> Struct {
        Struct { members }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "member")]
pub struct Member {
    name: MemberName,
    value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "name")]
struct MemberName {
    #[serde(rename = "$value")]
    name: String,
}

impl Member {
    pub fn new(name: String, value: Value) -> Member {
        Member {
            name: MemberName { name },
            value,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "array")]
pub struct Array {
    #[serde(default)]
    data: ArrayData,
}

impl Array {
    pub fn from_elements(values: Vec<Value>) -> Array {
        Array {
            data: ArrayData { values },
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "data")]
pub struct ArrayData {
    #[serde(default, rename = "value")]
    values: Vec<Value>,
}

#[cfg(test)]
mod tests;
