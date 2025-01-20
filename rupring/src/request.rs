/*!
# About Request
- You can access any value provided in an HTTP Request through the Request parameter.

```rust
#[rupring::Get(path = /:id)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    let method = request.method;
    assert_eq!(method, rupring::Method::GET);

    let path = request.path;
    assert_eq!(path, "/");

    let body = request.body;
    assert_eq!(body, "");

    let headers = request.headers;
    let content_type = headers.get("content-type").unwrap();
    assert_eq!(content_type, "text/plain");

    let id = request.path_parameters["id"].clone();
    assert_eq!(id, "123");

    let query = request.query_parameters["query"].clone();
    assert_eq!(query, vec!["asdf".to_string()]);

    //...

    response
}
```

## Request: Path Param

For path parameters, auto binding is provided through annotation.

The annotation name can be one of `Path`, `path`, or `PathVariable`.
```rust
#[rupring::Get(path = /echo/:id)]
pub fn echo(
    #[PathVariable="id"] id: i32
) -> rupring::Response {
    println!("id: {}", id);

    rupring::Response::new().text(request.body)
}
```

If the Path Param is optional, just wrap the type in `Option`.
```rust
#[rupring::Get(path = /echo/:id)]
pub fn echo(
    #[PathVariable="id"] id: Option<i32>
) -> rupring::Response {
    // ...

    rupring::Response::new().text("OK".to_string())
}
```

If you need Swagger documentation for the Path Param, you should add the `Description` annotation.
`Description` can also be used as `Desc`, `desc`, etc.
```rust
#[rupring::Get(path = /echo/:id)]
pub fn echo(
    #[path="id"] #[desc="asdf"] id: i32
) -> rupring::Response {
    println!("id: {}", id);

    rupring::Response::new().text(request.body)
}
```

If you want to define a custom type for PathParam, you can implement the ParamStringDeserializer trait.
```rust
struct SomeCustomType {}

impl rupring::ParamStringDeserializer<SomeCustomType> for rupring::ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<SomeCustomType, Self::Error> {
        //...
        Ok(SomeCustomType {})
    }
}
```
*/
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    panic::UnwindSafe,
    sync::Arc,
};

use hyper::header;

use crate::{
    core::multipart::{parse_multipart, parse_multipart_boundary},
    Method,
};

#[derive(Debug, Clone)]
pub struct Metadata {
    pub ip: IpAddr,
    pub header_size: usize,
    pub body_size: usize,
    pub number_of_headers: usize,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::from_bits(0)),
            header_size: 0,
            body_size: 0,
            number_of_headers: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub raw_body: Vec<u8>,
    pub files: Vec<MultipartFile>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub query_parameters: HashMap<String, Vec<String>>,
    pub path_parameters: HashMap<String, String>,
    pub metadata: Metadata,
    pub(crate) di_context: Arc<crate::DIContext>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipartFile {
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

impl Request {
    /*
    Parse the header value and save the file information in `Request::cookies`.
    There is no need to use this function if the `server.cookie.auto-parsing-enabled` option is true. (default true)
    */
    pub fn parse_cookies(&mut self) {
        if let Some(cookie_header) = self.headers.get(header::COOKIE.as_str()) {
            self.cookies = crate::core::cookie::parse_cookie_header(cookie_header);
        }
    }

    /*
    Parse the header value and request body value and save the file information in `Request::files`.
    There is no need to use this function if the `server.multipart.auto-parsing-enabled` option is true. (default true)
    */
    pub fn parse_multipart(&mut self) -> anyhow::Result<()> {
        for (header_name, header_value) in self.headers.iter() {
            if header_name == header::CONTENT_TYPE.as_str()
                && header_value.starts_with("multipart/form-data")
            {
                if let Some(multipart_boundary) = parse_multipart_boundary(header_value) {
                    self.files = parse_multipart(&self.raw_body, &multipart_boundary)?;
                    return Ok(());
                }

                return Err(anyhow::anyhow!("No boundary"));
            }
        }

        Err(anyhow::anyhow!("No multipart/form-data"))
    }
}

impl Request {
    pub fn bind<T: BindFromRequest + Default>(&self) -> anyhow::Result<T> {
        BindFromRequest::bind(self.clone())
    }
}

pub trait BindFromRequest {
    fn bind(request: Request) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl UnwindSafe for Request {}

impl Request {
    pub fn get_provider<T: 'static>(&self) -> Option<&T> {
        self.di_context.get::<T>()
    }
}

#[derive(Debug, Clone)]
pub struct QueryString(pub Vec<String>);

pub trait QueryStringDeserializer<T>: Sized {
    type Error;

    fn deserialize_query_string(&self) -> Result<T, Self::Error>;
}

impl<T> QueryStringDeserializer<Option<T>> for QueryString
where
    QueryString: QueryStringDeserializer<T>,
{
    type Error = ();

    fn deserialize_query_string(&self) -> Result<Option<T>, Self::Error> {
        let result = Self::deserialize_query_string(self);

        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}

impl QueryStringDeserializer<i8> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<i8, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<i8>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i16> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<i16, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<i16>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i32> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<i32, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<i32>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i64> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<i64, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<i64>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<i128> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<i128, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<i128>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<isize> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<isize, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<isize>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u8> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<u8, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<u8>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u16> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<u16, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<u16>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u32> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<u32, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<u32>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u64> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<u64, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<u64>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<u128> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<u128, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<u128>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<usize> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<usize, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<usize>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<f32> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<f32, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<f32>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<f64> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<f64, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<f64>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<bool> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<bool, Self::Error> {
        if let Some(e) = self.0.first() {
            e.parse::<bool>().map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl QueryStringDeserializer<String> for QueryString {
    type Error = ();

    fn deserialize_query_string(&self) -> Result<String, Self::Error> {
        if let Some(e) = self.0.first() {
            Ok(e.clone())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParamString(pub String);

pub trait ParamStringDeserializer<T>: Sized {
    type Error;

    fn deserialize(&self) -> Result<T, Self::Error>;
}

impl<T> ParamStringDeserializer<Option<T>> for ParamString
where
    ParamString: ParamStringDeserializer<T>,
{
    type Error = ();

    fn deserialize(&self) -> Result<Option<T>, Self::Error> {
        let result = Self::deserialize(self);
        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}

impl ParamStringDeserializer<i8> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i8, Self::Error> {
        self.0.parse::<i8>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i16> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i16, Self::Error> {
        self.0.parse::<i16>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i32> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i32, Self::Error> {
        self.0.parse::<i32>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i64> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i64, Self::Error> {
        self.0.parse::<i64>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<i128> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<i128, Self::Error> {
        self.0.parse::<i128>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<isize> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<isize, Self::Error> {
        self.0.parse::<isize>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u8> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u8, Self::Error> {
        self.0.parse::<u8>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u16> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u16, Self::Error> {
        self.0.parse::<u16>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u32> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u32, Self::Error> {
        self.0.parse::<u32>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u64> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u64, Self::Error> {
        self.0.parse::<u64>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<u128> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<u128, Self::Error> {
        self.0.parse::<u128>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<usize> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<usize, Self::Error> {
        self.0.parse::<usize>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<f32> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<f32, Self::Error> {
        self.0.parse::<f32>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<f64> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<f64, Self::Error> {
        self.0.parse::<f64>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<bool> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<bool, Self::Error> {
        self.0.parse::<bool>().map_err(|_| ())
    }
}

impl ParamStringDeserializer<String> for ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<String, Self::Error> {
        Ok(self.0.clone())
    }
}
