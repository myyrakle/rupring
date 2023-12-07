use std::{collections::HashMap, net::SocketAddr};

use hyper::Method;
extern crate rupring;

#[derive(Debug, Clone, Copy)]
pub struct RootModule {}

impl rupring::IModule for RootModule {
    fn child_modules(&self) -> Vec<Box<dyn rupring::IModule>> {
        vec![]
    }

    fn controllers(&self) -> Vec<Box<dyn rupring::IController>> {
        vec![Box::new(HomeController {})]
    }
}

#[derive(Debug, Clone)]
pub struct HomeController {}

impl rupring::IController for HomeController {
    fn prefix(&self) -> String {
        "".to_string()
    }

    fn routes(&self) -> Vec<Box<dyn rupring::IRoute>> {
        vec![Box::new(ARoute {}), Box::new(BRoute {})]
    }
}

#[derive(Debug, Clone)]
pub struct ARoute {}

impl rupring::IRoute for ARoute {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "/hello".to_string()
    }

    fn handler(&self) -> Box<dyn rupring::IHandler> {
        Box::new(ARouteHandler {})
    }
}

#[derive(Debug, Clone)]
pub struct ARouteHandler {}

impl rupring::IHandler for ARouteHandler {
    fn handle(&self, request: rupring::Request) -> rupring::Response {
        rupring::Response {
            status: 200,
            body: "Hello, World!".to_string(),
            headers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BRoute {}

impl rupring::IRoute for BRoute {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "/boom".to_string()
    }

    fn handler(&self) -> Box<dyn rupring::IHandler> {
        Box::new(BRouteHandler {})
    }
}

#[derive(Debug, Clone)]
pub struct BRouteHandler {}

impl rupring::IHandler for BRouteHandler {
    fn handle(&self, request: rupring::Request) -> rupring::Response {
        rupring::Response {
            status: 400,
            body: "BOOM!".to_string(),
            headers: HashMap::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let module = RootModule {};
    rupring::boot::run_server(addr, module).await?;

    Ok(())
}