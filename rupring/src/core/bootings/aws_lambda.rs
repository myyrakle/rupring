use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

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

    println!("@ 1");

    println!("aws_lambda_runtime_api: {}", aws_lambda_runtime_api);

    let url = format!("http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/next");

    let stream = TcpStream::connect(aws_lambda_runtime_api).await?;

    println!("@ 2");

    let (mut sender, connection) =
        hyper::client::conn::http1::handshake(TokioIo::new(stream)).await?;

    println!("@ 3");

    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            println!("Connection failed: {:?}", err);
        }
    });

    println!("@ 4");

    let hyper_request = hyper::Request::builder()
        .uri(url)
        .header(hyper::header::HOST, "localhost")
        .body(Empty::<Bytes>::new())
        .unwrap();

    println!("@ 5");

    let mut response = sender.send_request(hyper_request).await?;

    println!("@ 6");

    let mut request_context = RequestContext::default();

    let headers = response.headers();

    if let Some(aws_request_id) = headers.get("Lambda-Runtime-Aws-Request-Id") {
        request_context.aws_request_id = aws_request_id.to_str()?.to_string();
    }

    let response_body = match response.body_mut().collect().await {
        Ok(body) => {
            let body = body.to_bytes();
            let body = String::from_utf8(body.to_vec()).unwrap_or("".to_string());

            body
        }
        Err(err) => {
            return Err(anyhow::anyhow!("Failed to read response body: {:?}", err));
        }
    };

    request_context.response_body = response_body;

    Ok(request_context)
}
