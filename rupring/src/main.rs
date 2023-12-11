#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[], providers=[])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, echo])]
pub struct HomeController {}

#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response {
        status: 200,
        body: "Hello, World!".to_string(),
        headers: Default::default(),
    }
}

#[rupring::Get(path = /echo)]
pub fn echo(request: rupring::Request) -> rupring::Response {
    rupring::Response {
        status: 200,
        body: request.body,
        headers: Default::default(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::net::SocketAddr;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let module = RootModule {};
    rupring::boot::run_server(addr, module).await?;

    Ok(())
}
