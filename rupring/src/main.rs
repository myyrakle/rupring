use std::any::TypeId;

pub struct HomeService {}

impl HomeService {
    pub fn hello(&self) -> String {
        "Hello, World!".to_string()
    }
}

impl rupring::Provider for HomeService {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn provide(&self, _di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        Box::new(HomeService {})
    }
}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[], providers=[
    HomeService{},
])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, echo])]
pub struct HomeController {}

#[rupring::Get(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    let home_service = request.di_context.get::<HomeService>().unwrap();

    rupring::Response {
        status: 200,
        body: home_service.hello(),
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
