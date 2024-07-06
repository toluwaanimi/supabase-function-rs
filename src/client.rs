use crate::errors::{FunctionsError};
use crate::models::{FunctionInvokeOptions, FunctionRegion, FunctionsResponse, HttpMethod, InvokeBody, ResponseData};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct FunctionsClient {
    url: String,
    headers: HashMap<String, String>,
    region: FunctionRegion,
    client: Client,
}

impl FunctionsClient {
    pub fn new(url: String, headers: Option<HashMap<String, String>>, region: Option<FunctionRegion>) -> Self {
        Self {
            url,
            headers: headers.unwrap_or_default(),
            region: region.unwrap_or(FunctionRegion::Any),
            client: Client::new(),
        }
    }

    pub fn set_auth(&mut self, token: String) {
        self.headers.insert("Authorization".to_string(), format!("Bearer {}", token));
    }

    pub async fn invoke(
        &self,
        function_name: &str,
        options: Option<FunctionInvokeOptions>,
    ) -> Result<FunctionsResponse, FunctionsError> {
        let options = options.unwrap_or_default();
        let headers = self.headers.clone();

        let mut req_headers = HeaderMap::new();
        for (key, value) in headers {
            req_headers.insert(
                HeaderName::try_from(key.as_str()).map_err(|_| FunctionsError::FetchError("Invalid header name".into()))?,
                HeaderValue::from_str(&value).map_err(|_| FunctionsError::FetchError("Invalid header value".into()))?,
            );
        }

        if let Some(region) = options.region {
            if region != FunctionRegion::Any {
                req_headers.insert(
                    HeaderName::from_static("x-region"),
                    HeaderValue::from_str(region.to_string().as_str()).map_err(|_| FunctionsError::FetchError("Invalid region value".into()))?,
                );
            }
        }

        let method = options.method.unwrap_or(HttpMethod::Post);
        let method_str = method.as_str();
        let url = format!("{}/{}", self.url, function_name);


        let request_builder = match options.body {
            Some(InvokeBody::File(ref file)) |
            Some(InvokeBody::Blob(ref file)) |
            Some(InvokeBody::ArrayBuffer(ref file)) => {
                req_headers.insert("Content-Type", HeaderValue::from_static("application/octet-stream"));
                self.client.request(method_str.parse().unwrap(), &url).headers(req_headers).body(file.clone())
            }
            Some(InvokeBody::String(ref s)) => {
                req_headers.insert("Content-Type", HeaderValue::from_static("text/plain"));
                self.client.request(method_str.parse().unwrap(), &url).headers(req_headers).body(s.clone())
            }
            Some(InvokeBody::FormData(ref form_data)) => {
                let form = reqwest::multipart::Form::new();
                let form = form_data.iter().fold(form, |form, (key, value)| {
                    form.text(key.clone(), value.clone())
                });
                self.client.request(method_str.parse().unwrap(), &url).headers(req_headers).multipart(form)
            }
            Some(InvokeBody::Json(ref json)) => {
                req_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
                self.client.request(method_str.parse().unwrap(), &url).headers(req_headers).json(json)
            }
            None => self.client.request(method_str.parse().unwrap(), &url).headers(req_headers),
        };

        let response = request_builder.send().await.map_err(|e| FunctionsError::FetchError(e.to_string()))?;


        if let Some(is_relay_error) = response.headers().get("x-relay-error") {
            if is_relay_error == "true" {
                return Err(FunctionsError::RelayError("Relay Error invoking the Edge Function".into()));
            }
        }

        if !response.status().is_success() {
            return Err(FunctionsError::HttpError(response.status().to_string()));
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("text/plain")
            .split(';')
            .next()
            .unwrap_or("text/plain");

        let data = match content_type {
            "application/json" => {
                let json_data = response.json::<serde_json::Value>().await.map_err(|e| FunctionsError::FetchError(e.to_string()))?;
                ResponseData::Json(json_data)
            },
            "application/octet-stream" => {
                let bytes_data = response.bytes().await.map_err(|e| FunctionsError::FetchError(e.to_string()))?;
                ResponseData::Bytes(bytes_data)
            },
            "text/event-stream" => {
                let text_data = response.text().await.map_err(|e| FunctionsError::FetchError(e.to_string()))?;
                ResponseData::Text(text_data)
            },
            "multipart/form-data" => {
                let form_data = response.json::<HashMap<String, String>>().await.map_err(|e| FunctionsError::FetchError(e.to_string()))?;
                ResponseData::FormData(form_data)
            },
            _ => {
                let text_data = response.text().await.map_err(|e| FunctionsError::FetchError(e.to_string()))?;
                ResponseData::Text(text_data)
            }
        };

        Ok(FunctionsResponse::Success { data })
    }
}
