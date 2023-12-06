use std::collections::HashMap;

mod boot;
pub use rupring_macro::{Controller, Delete, Get, Injectable, Module, Patch, Post, Put};

pub struct Request {
    pub method: String,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, Vec<String>>,
    pub query: HashMap<String, Vec<String>>,
}

pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, Vec<String>>,
}

pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
}

pub trait IController {
    fn prefix(&self) -> String;
    fn routes(&self) -> Vec<Box<dyn IRoute>>;
}

pub trait IRoute {
    fn method(&self) -> String;
    fn path(&self) -> String;
    fn handler(&self) -> Box<dyn IHandler>;
}

pub trait IHandler {
    fn handle(&self, request: Request, controller: Box<dyn IController>) -> Response;
}

#[derive(Debug, Clone)]
pub struct Rupring<T: IModule> {
    root_module: T,
}

impl<T: IModule> Rupring<T> {
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
