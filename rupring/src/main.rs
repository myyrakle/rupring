use std::{collections::HashMap, net::SocketAddr};

use hyper::Method;

#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, boom])]
pub struct HomeController {}

#[rupring::Get(path = /hello)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    rupring::Response {
        status: 200,
        body: "Hello, World!".to_string(),
        headers: HashMap::new(),
    }
}

#[rupring::Get(path = /boom)]
pub fn boom(request: rupring::Request) -> rupring::Response {
    rupring::Response {
        status: 400,
        body: "boom!".to_string(),
        headers: HashMap::new(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let module = RootModule {};
    rupring::boot::run_server(addr, module).await?;

    Ok(())
}
