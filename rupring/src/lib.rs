pub mod boot;
pub mod request;
pub mod response;

pub use rupring_macro::{Controller, Delete, Get, Module, Patch, Post, Put};

extern crate rupring_macro;

pub type Method = hyper::Method;

pub use boot::di::DIContext;
pub use boot::di::Provider;
pub use request::Request;
pub use response::Response;

pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
    fn providers(&self) -> Vec<Box<dyn Provider>>;
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
