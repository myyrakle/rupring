use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use rupring::{NextFunction};

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

// impl rupring::IProvider for CounterService {
//     fn dependencies(&self) -> Vec<TypeId> {
//         vec![]
//     }

//     fn provide(&self, _di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
//         Box::new(CounterService::new())
//     }
// }

#[rupring_macro::Injectable(CounterServiceFactory)]
fn inject_counter_service() -> CounterService {
    CounterService::new()
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

pub fn logger_middleware(
    request: rupring::Request,
    response: rupring::Response,
    next: NextFunction,
) -> rupring::Response {
    println!(
        "Request: {} {}",
        request.method.to_string(),
        request.path.to_string()
    );

    next(request, response)
}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[HomeController{}], 
    modules=[UserModule{}, rupring::swagger::module::SwaggerModule{}], 
    providers=[CounterServiceFactory{}], 
    middlewares=[]
)]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, count, go_to_naver, foo::echo, echo])]
pub struct HomeController {}

#[rupring::Get(path = /)]
#[summary = "기본 root API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [home]]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("123214")
}

#[rupring::Get(path = /user)]
#[tags = [user]]
pub fn get_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    let user_service = request.get_provider::<Box<dyn IUserService>>().unwrap();

    rupring::Response::new().text(user_service.get_user())
}

mod foo {
    #[rupring::Get(path = /echo2)]
    pub fn echo(request: rupring::Request, _: rupring::Response) -> rupring::Response {
        rupring::Response::new().text(request.body)
    }
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

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[UserController{}],
    modules=[],
    providers=[
        UserService::default()
    ],
    middlewares=[]
)]
pub struct UserModule {}

#[rupring::Get(path = /go-to-naver)]
pub fn go_to_naver(_: rupring::Request, _: rupring::Response) -> rupring::Response {
    rupring::Response::new().redirect("https://naver.com")
}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[get_user], middlewares=[])]
pub struct UserController {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = rupring::RupringFactory::create(RootModule {});

    app.listen(3000).await?;

    Ok(())
}
