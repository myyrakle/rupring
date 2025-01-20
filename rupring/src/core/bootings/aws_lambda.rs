use std::{collections::HashMap, str::FromStr};

use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap, Uri,
};
use serde::{Deserialize, Serialize};

use crate::utils;

// Reference: https://docs.aws.amazon.com/ko_kr/lambda/latest/dg/runtimes-api.html
fn get_aws_lambda_runtime_api() -> Option<String> {
    std::env::var("AWS_LAMBDA_RUNTIME_API").ok()
}

#[derive(Debug, Default, Clone)]
pub struct LambdaRequestEvent {
    pub aws_request_id: String,
    pub trace_id: String,
    pub status_code: u16,

    pub event_payload: LambdaRequestPayload,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LambdaRequestHTTP {
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LambdaRequestContext {
    #[serde(rename = "http")]
    pub http: LambdaRequestHTTP,
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct LambdaRequestPayload {
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,
    #[serde(rename = "body")]
    pub body: Option<String>,
    #[serde(rename = "rawPath")]
    pub raw_path: String,
    #[serde(rename = "rawQueryString")]
    pub raw_query_string: String,
    #[serde(rename = "requestContext")]
    pub request_context: LambdaRequestContext,
}

impl LambdaRequestPayload {
    pub fn to_hyper_headermap(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        for (key, value) in self.headers.iter() {
            if let Ok(header_name) = HeaderName::from_str(key) {
                if let Ok(value) = HeaderValue::from_str(value) {
                    headers.insert(header_name, value);
                }
            }
        }

        headers
    }

    pub fn to_full_url(&self) -> String {
        let domain = self.request_context.domain_name.as_str();
        let path = self.raw_path.as_str();

        let query_string = if self.raw_query_string.is_empty() {
            "".to_string()
        } else {
            format!("?{}", self.raw_query_string.as_str())
        };

        format!("http://{domain}{path}{query_string}")
    }
}

pub async fn get_request_context() -> anyhow::Result<LambdaRequestEvent> {
    let aws_lambda_runtime_api = match get_aws_lambda_runtime_api() {
        Some(api) => api,
        None => return Err(anyhow::anyhow!("AWS_LAMBDA_RUNTIME_API is not set")),
    };

    let url = Uri::from_str(
        format!("http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/next").as_str(),
    )?;

    let mut headers = HashMap::new();
    headers.insert(hyper::header::HOST, "localhost".to_owned());

    let response =
        utils::hyper::send_http_request(url, hyper::Method::GET, headers, "".to_owned()).await?;

    let mut request_context = LambdaRequestEvent::default();

    if let Some(aws_request_id) = response.headers.get("Lambda-Runtime-Aws-Request-Id") {
        request_context.aws_request_id = aws_request_id.to_str()?.to_string();
    }

    if let Some(trace_id) = response.headers.get("Lambda-Runtime-Trace-Id") {
        request_context.trace_id = trace_id.to_str()?.to_string();
    }

    request_context.event_payload = serde_json::from_str(&response.body)?;
    request_context.status_code = response.status_code;

    Ok(request_context)
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct LambdaReponse {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    #[serde(rename = "headers")]
    pub headers: HashMap<String, String>,
    #[serde(rename = "body")]
    pub body: String,
}

pub async fn send_response_to_lambda(
    aws_request_id: &str,
    response: LambdaReponse,
) -> anyhow::Result<()> {
    let response = serde_json::to_string(&response)?;

    let aws_lambda_runtime_api = match get_aws_lambda_runtime_api() {
        Some(api) => api,
        None => return Err(anyhow::anyhow!("AWS_LAMBDA_RUNTIME_API is not set")),
    };

    let url = Uri::from_str(
        format!(
            "http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/{aws_request_id}/response"
        )
        .as_str(),
    )?;

    let mut headers = HashMap::new();
    headers.insert(hyper::header::HOST, "localhost".to_owned());

    let _ = utils::hyper::send_http_request(url, hyper::Method::POST, headers, response).await?;

    Ok(())
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct LambdaError {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(rename = "stackTrace")]
    pub stack_trace: Vec<String>,
}

pub async fn send_error_to_lambda(aws_request_id: &str, error: LambdaError) -> anyhow::Result<()> {
    let error = serde_json::to_string(&error)?;

    let aws_lambda_runtime_api = match get_aws_lambda_runtime_api() {
        Some(api) => api,
        None => return Err(anyhow::anyhow!("AWS_LAMBDA_RUNTIME_API is not set")),
    };

    let url = Uri::from_str(
        format!(
            "http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/{aws_request_id}/error"
        )
        .as_str(),
    )?;

    let mut headers = HashMap::new();
    headers.insert(hyper::header::HOST, "localhost".to_owned());

    let _ = utils::hyper::send_http_request(url, hyper::Method::POST, headers, error).await?;

    Ok(())
}
