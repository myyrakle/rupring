pub(crate) mod boot;
pub(crate) mod request;
pub(crate) mod response;

pub use rupring_macro::{Controller, Delete, Get, Module, Patch, Post, Put};

extern crate rupring_macro;

pub type Method = hyper::Method;

/// Dependency Injection Context for entire life cycle
pub use boot::di::DIContext;
/// Dependency Injection Provider
pub use boot::di::IProvider;
/// Request
pub use request::Request;
/// Response
pub use response::Response;

/// Module interface
pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
    fn providers(&self) -> Vec<Box<dyn IProvider>>;
}

/// Controller interface
pub trait IController {
    fn prefix(&self) -> String;
    fn routes(&self) -> Vec<Box<dyn IRoute + Send + 'static>>;
}

/// Route interface
pub trait IRoute {
    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn handler(&self) -> Box<dyn IHandler + Send + 'static>;
}

/// Handler interface
pub trait IHandler {
    fn handle(&self, request: Request) -> Response;
}

#[derive(Debug, Clone)]
pub struct RupringFactory<T: IModule> {
    root_module: T,
}

impl<T: IModule + Clone + Copy + Sync + Send + 'static> RupringFactory<T> {
    pub fn create(module: T) -> Self {
        RupringFactory {
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
