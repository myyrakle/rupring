use http_body_util::{BodyExt, Limited};

pub trait RequestAdapter {
    fn uri(&self) -> &hyper::Uri;
    fn method(&self) -> &hyper::Method;
    fn http_version(&self) -> hyper::Version {
        hyper::Version::HTTP_11
    }
    fn headers(&self) -> &hyper::HeaderMap;
    async fn body(self, limit: usize) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct HyperRequest(pub hyper::Request<hyper::body::Incoming>);

impl RequestAdapter for HyperRequest {
    fn uri(&self) -> &hyper::Uri {
        self.0.uri()
    }

    fn method(&self) -> &hyper::Method {
        self.0.method()
    }

    fn http_version(&self) -> hyper::Version {
        self.0.version()
    }

    fn headers(&self) -> &hyper::HeaderMap {
        self.0.headers()
    }

    async fn body(self, limit: usize) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let limited_request_body_stream = Limited::new(self.0, limit);

        let bytes = limited_request_body_stream.collect().await?;

        Ok(bytes.to_bytes().to_vec())
    }
}

#[derive(Debug, Clone)]
pub struct AWSLambdaRequest {
    pub uri: hyper::Uri,
    pub method: hyper::Method,
    pub http_version: hyper::Version,
    pub headers: hyper::HeaderMap,
    pub body: Vec<u8>,
}

impl RequestAdapter for AWSLambdaRequest {
    fn uri(&self) -> &hyper::Uri {
        &self.uri
    }

    fn method(&self) -> &hyper::Method {
        &self.method
    }

    fn http_version(&self) -> hyper::Version {
        self.http_version
    }

    fn headers(&self) -> &hyper::HeaderMap {
        &self.headers
    }

    async fn body(
        self,
        _limit: usize,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.body)
    }
}
