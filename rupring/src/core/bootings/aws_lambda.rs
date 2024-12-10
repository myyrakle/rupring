use std::{collections::HashMap, str::FromStr};

use hyper::Uri;

use crate::utils;

// Reference: https://docs.aws.amazon.com/ko_kr/lambda/latest/dg/runtimes-api.html
fn get_aws_lambda_runtime_api() -> Option<String> {
    std::env::var("AWS_LAMBDA_RUNTIME_API").ok()
}

#[derive(Debug, Default, Clone)]
pub struct RequestContext {
    pub aws_request_id: String,
    pub response_body: String,
}

pub async fn get_request_context() -> anyhow::Result<RequestContext> {
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

    let mut request_context = RequestContext::default();

    if let Some(aws_request_id) = response.headers.get("Lambda-Runtime-Aws-Request-Id") {
        request_context.aws_request_id = aws_request_id.to_str()?.to_string();
    }

    request_context.response_body = response.body;

    Ok(request_context)
}

pub async fn send_response_to_lambda(
    aws_request_id: String,
    response: String,
) -> anyhow::Result<()> {
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
