use std::collections::HashMap;

use http_body_util::BodyExt;
use hyper::{header::HeaderName, Method, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[allow(dead_code)]
pub struct HTTPResponse {
    pub status_code: u16,
    pub headers: hyper::HeaderMap,
    pub body: String,
}

#[allow(dead_code)]
pub async fn send_http_request(
    url: Uri,
    method: Method,
    headers: HashMap<HeaderName, String>,
    request_body: String,
) -> anyhow::Result<HTTPResponse> {
    let host = url
        .authority()
        .ok_or(anyhow::anyhow!("host:port is not set"))?;

    let stream = TcpStream::connect(host.as_str()).await?;

    let (mut sender, connection) =
        hyper::client::conn::http1::handshake(TokioIo::new(stream)).await?;

    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let mut hyper_request = hyper::Request::builder().method(method).uri(&url);

    for (key, value) in headers.iter() {
        hyper_request = hyper_request.header(key, value);
    }

    let hyper_request = hyper_request.body(request_body).unwrap();

    let mut response = sender.send_request(hyper_request).await?;

    let headers = response.headers().to_owned();

    let response_body = match response.body_mut().collect().await {
        Ok(body) => {
            let body = body.to_bytes();
            String::from_utf8(body.to_vec()).unwrap_or("".to_string())
        }
        Err(err) => {
            return Err(anyhow::anyhow!("Failed to read response body: {:?}", err));
        }
    };

    println!("{url:?}, !!!, {}", response_body);

    Ok(HTTPResponse {
        status_code: response.status().as_u16(),
        headers,
        body: response_body,
    })
}
