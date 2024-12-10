use http_body_util::BodyExt;
use hyper::{Method, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub struct HTTPResponse {
    pub status_code: u16,
    pub headers: hyper::HeaderMap,
    pub body: String,
}

pub async fn send_http_request(
    url: Uri,
    method: Method,
    request_body: String,
) -> anyhow::Result<HTTPResponse> {
    let host = url.host().ok_or(anyhow::anyhow!("host is not set"))?;

    let stream = TcpStream::connect(host).await?;

    let (mut sender, connection) =
        hyper::client::conn::http1::handshake(TokioIo::new(stream)).await?;

    tokio::task::spawn(async move {
        if let Err(err) = connection.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let hyper_request = hyper::Request::builder()
        .method(method)
        .uri(url)
        .header(hyper::header::HOST, "localhost")
        .body(request_body)
        .unwrap();

    let mut response = sender.send_request(hyper_request).await?;

    let headers = response.headers().to_owned();

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

    Ok(HTTPResponse {
        status_code: response.status().as_u16(),
        headers,
        body: response_body,
    })
}
