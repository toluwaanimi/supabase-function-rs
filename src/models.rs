use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use crate::errors::FunctionsError;
use bytes::Bytes;
use serde::de::{self, Visitor, MapAccess};
use serde::{Deserialize, Serialize, Serializer, Deserializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionRegion {
    Any,
    ApNortheast1,
    ApNortheast2,
    ApSouth1,
    ApSoutheast1,
    ApSoutheast2,
    CaCentral1,
    EuCentral1,
    EuWest1,
    EuWest2,
    EuWest3,
    SaEast1,
    UsEast1,
    UsWest1,
    UsWest2,
}

impl Display for FunctionRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FunctionRegion::Any => "any".to_string(),
            FunctionRegion::ApNortheast1 => "ap-northeast-1".to_string(),
            FunctionRegion::ApNortheast2 => "ap-northeast-2".to_string(),
            FunctionRegion::ApSouth1 => "ap-south-1".to_string(),
            FunctionRegion::ApSoutheast1 => "ap-southeast-1".to_string(),
            FunctionRegion::ApSoutheast2 => "ap-southeast-2".to_string(),
            FunctionRegion::CaCentral1 => "ca-central-1".to_string(),
            FunctionRegion::EuCentral1 => "eu-central-1".to_string(),
            FunctionRegion::EuWest1 => "eu-west-1".to_string(),
            FunctionRegion::EuWest2 => "eu-west-2".to_string(),
            FunctionRegion::EuWest3 => "eu-west-3".to_string(),
            FunctionRegion::SaEast1 => "sa-east-1".to_string(),
            FunctionRegion::UsEast1 => "us-east-1".to_string(),
            FunctionRegion::UsWest1 => "us-west-1".to_string(),
            FunctionRegion::UsWest2 => "us-west-2".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FunctionInvokeOptions {
    pub headers: Option<HashMap<String, String>>,
    pub method: Option<HttpMethod>,
    pub region: Option<FunctionRegion>,
    pub body: Option<InvokeBody>,
}

#[derive(Debug, Clone)]
pub enum InvokeBody {
    File(Vec<u8>),
    Blob(Vec<u8>),
    ArrayBuffer(Vec<u8>),
    FormData(HashMap<String, String>),
    Json(HashMap<String, serde_json::Value>),
    String(String),
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            HttpMethod::Post => "POST",
            HttpMethod::Get => "GET",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub enum ResponseData {
    Json(serde_json::Value),
    Text(String),
    #[serde(serialize_with = "serialize_bytes")]
    Bytes(Bytes),
    FormData(HashMap<String, String>),
}

// Implement custom deserialization for ResponseData
impl<'de> Deserialize<'de> for ResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ResponseDataVisitor)
    }
}

struct ResponseDataVisitor;

impl<'de> Visitor<'de> for ResponseDataVisitor {
    type Value = ResponseData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid ResponseData variant")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        while let Some(key) = map.next_key()? {
            match key {
                "Json" => {
                    let value = map.next_value()?;
                    return Ok(ResponseData::Json(value));
                }
                "Text" => {
                    let value = map.next_value()?;
                    return Ok(ResponseData::Text(value));
                }
                "Bytes" => {
                    let value: Vec<u8> = map.next_value()?;
                    return Ok(ResponseData::Bytes(Bytes::from(value)));
                }
                "FormData" => {
                    let value = map.next_value()?;
                    return Ok(ResponseData::FormData(value));
                }
                _ => return Err(de::Error::unknown_field(key, FIELDS)),
            }
        }
        Err(de::Error::custom("missing fields"))
    }
}

const FIELDS: &'static [&'static str] = &["Json", "Text", "Bytes", "FormData"];

// Custom serializer for Bytes
fn serialize_bytes<S>(bytes: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(bytes)
}


#[derive(Debug)]
pub enum FunctionsResponse {
    Success { data: ResponseData },
    Failure { error: FunctionsError },
}
