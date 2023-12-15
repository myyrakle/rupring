use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
pub struct CounterService {
    counter: Arc<Mutex<i32>>,
}

impl CounterService {
    pub fn new() -> Self {
        CounterService {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }

    pub fn get(&self) -> i32 {
        let counter = self.counter.lock().unwrap();
        *counter
    }
}

impl rupring::IProvider for CounterService {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn provide(&self, _di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        Box::new(CounterService::new())
    }
}

pub trait IUserService {
    fn get_user(&self) -> String;
}

#[derive(Debug, Clone, Default)]
pub struct UserService {}

impl IUserService for UserService {
    fn get_user(&self) -> String {
        "user".to_string()
    }
}

impl rupring::IProvider for UserService {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn provide(&self, _di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        let service: Box<dyn IUserService> = Box::new(UserService::default());
        return Box::new(service);
    }
}

#[derive(Debug, Clone, Default)]
pub struct HomeService {
    home_repository: HomeRepository,
}

impl HomeService {
    pub fn hello(&self) -> String {
        self.home_repository.hello()
    }
}

impl rupring::IProvider for HomeService {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<HomeRepository>()]
    }

    fn provide(&self, di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        Box::new(HomeService {
            home_repository: di_context.get::<HomeRepository>().unwrap().to_owned(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct HomeRepository {}

impl HomeRepository {
    pub fn hello(&self) -> String {
        "Hello, World!".to_string()
    }
}

impl rupring::IProvider for HomeRepository {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn provide(&self, _di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        Box::new(HomeRepository::default())
    }
}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[], providers=[
    HomeService::default(), HomeRepository::default(), UserService::default(), CounterService::default()
])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, get_user, echo, count])]
pub struct HomeController {}

#[rupring::Get(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    let home_service = request.get_provider::<HomeService>().unwrap();

    rupring::Response::new().text(home_service.hello())
}

#[rupring::Get(path = /user)]
pub fn get_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    let user_service = request.get_provider::<Box<dyn IUserService>>().unwrap();

    rupring::Response::new().text(user_service.get_user())
}

#[rupring::Get(path = /echo)]
pub fn echo(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    rupring::Response::new().text(request.body)
}

#[rupring::Get(path = /count)]
pub fn count(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    let counter_service = request.get_provider::<CounterService>().unwrap();
    counter_service.increment();
    let count = counter_service.get();

    rupring::Response::new().text(format!("{}", count))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = rupring::RupringFactory::create(RootModule {});

    app.listen(3000).await?;

    Ok(())
}
