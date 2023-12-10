use std::collections::HashMap;

pub mod boot;
use http_body_util::Full;
use hyper::{body::Bytes, header::HeaderName};
pub use rupring_macro::{Controller, Delete, Get, Injectable, Module, Patch, Post, Put};

extern crate rupring_macro;

pub type Method = hyper::Method;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, Vec<String>>,
    pub path_parameters: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<HeaderName, String>,
}

impl From<Response> for hyper::Response<Full<Bytes>> {
    fn from(response: Response) -> Self {
        let mut builder = hyper::Response::builder();

        builder = builder.status(response.status);

        for (header_name, header_value) in response.headers {
            builder = builder.header(header_name.clone(), header_value);
        }

        let response = builder.body(Full::new(Bytes::from(response.body))).unwrap();

        return response;
    }
}

pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
}

pub trait IController {
    fn prefix(&self) -> String;
    fn routes(&self) -> Vec<Box<dyn IRoute + Send + 'static>>;
}

pub trait IRoute {
    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn handler(&self) -> Box<dyn IHandler + Send + 'static>;
}

pub trait IHandler {
    fn handle(&self, request: Request) -> Response;
}

#[derive(Debug, Clone)]
pub struct Rupring<T: IModule> {
    root_module: T,
}

impl<T: IModule + Clone + Copy + Sync + Send + 'static> Rupring<T> {
    pub fn create(module: T) -> Self {
        Rupring {
            root_module: module,
        }
    }

    pub async fn listen(self, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

        let result = boot::run_server(socket_addr, self.root_module).await;

        return result;
    }
}
