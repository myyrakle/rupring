/*! # Get Started
There is only one dependency.
```
cargo add rupring
```

And you can write your server like this:
```
#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, echo])]
pub struct HomeController {}

#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("Hello, World!".to_string())
}

#[rupring::Get(path = /echo)]
pub fn echo(request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text(request.body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = rupring::RupringFactory::create(RootModule {});

    app.listen(3000).await?;

    Ok(())
}

```

# Response

You can create a response like this:
```
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("Hello, World!".to_string())
}
```

You can also return a json value like this:
```
#[derive(serde::Serialize)]
struct User {
    name: String,
}

#[rupring::Get(path = /user)]
pub fn get_user(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().json(User {
        name: "John".to_string(),
    })
}
```

You can set the status code like this:
```
#[rupring::Get(path = /asdf)]
pub fn not_found(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("not found".to_string()).status(404)
}
```

You can set the header like this:
```
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text("Hello, World!".to_string())
        .header("content-type", "text/plain".to_string())
}
```

If you want, you can receive it as a parameter instead of creating the response directly.
```
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request, response: rupring::Response) -> rupring::Response {
    response
        .text("Hello, World!".to_string())
        .header("content-type", "text/plain".to_string())
}
```
This is especially useful when you need to inherit and use Response through middleware.


# Dependency Injection

rupring provides DI feature.

If you want to implement and DI a Provider that contains simple functionality, you can do it like this:
First, define the Provider.
```
#[derive(Debug, Clone, Default)]
pub struct HomeService {}

impl HomeService {
    pub fn hello(&self) -> String {
        "hello!!".to_string()
    }
}

impl rupring::IProvider for HomeService {
    fn provide(&self, di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        Box::new(HomeService {})
    }
}
```

Second, add it as a dependency to the module you want to use.
```
#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[], providers=[HomeService::default()])]
pub struct RootModule {}
```

And, you can use it by getting it from the router through the request object.
```
#[rupring::Get(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    let home_service = request.get_provider::<HomeService>().unwrap();

    rupring::Response::new().text(home_service.hello())
}
```

If a provider requires another provider, you must specify the dependency cycle as follows:
```
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
```

If you need mutables within the provider, you must ensure thread safety through Mutex or Atomic as follows:
```
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
```

If you need to abstract based on a trait, you need to box it twice as follows:

```
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

// ...

#[rupring::Get(path = /user)]
pub fn get_user(request: rupring::Request) -> rupring::Response {
    let user_service = request.get_provider::<Box<dyn IUserService>>().unwrap();

    rupring::Response::new().text(user_service.get_user())
}
```
*/

pub(crate) mod boot;
pub(crate) mod request;
pub(crate) mod response;

use std::panic::UnwindSafe;

/// Controller Annotation
/// ```
/// #[derive(Debug, Clone)]
/// #[rupring::Controller(prefix=/, routes=[hello, echo])]
/// pub struct HomeController {}
/// ```
pub use rupring_macro::Controller;

/// Module Annotation
/// ```
/// #[derive(Debug, Clone, Copy)]
/// #[rupring::Module(
///    controllers=[HomeController{}],
///    modules=[],
///    providers=[HomeService::default(), HomeRepository::default(), UserService::default(), CounterService::default()]
/// )]
/// pub struct RootModule {}
pub use rupring_macro::Module;

/// Get Route Annotation
/// ```
/// #[rupring::Get(path = /)]
/// pub fn hello(request: rupring::Request) -> rupring::Response {
///    // ...
/// }
pub use rupring_macro::Get;

/// Post Route Annotation
/// ```
/// #[rupring::Post(path = /)]
/// pub fn hello(request: rupring::Request) -> rupring::Response {
///   // ...
/// }
pub use rupring_macro::Post;

/// Patch Route Annotation
/// ```
/// #[rupring::Patch(path = /)]
/// pub fn hello(request: rupring::Request) -> rupring::Response {
///   // ...
/// }
pub use rupring_macro::Patch;

/// Put Route Annotation
/// ```
/// #[rupring::Put(path = /)]
/// pub fn hello(request: rupring::Request) -> rupring::Response {
///   // ...
/// }
pub use rupring_macro::Put;

/// Delete Route Annotation
/// ```
/// #[rupring::Delete(path = /)]
/// pub fn hello(request: rupring::Request) -> rupring::Response {
///     // ...
/// }
pub use rupring_macro::Delete;

extern crate rupring_macro;

/// HTTP method (from hyper crate)
pub type Method = hyper::Method;

/// HTTP Header Name (from hyper crate)
pub type HeaderName = hyper::header::HeaderName;

/// Dependency Injection Context for entire life cycle
pub use boot::di::DIContext;
/// Dependency Injection Provider
pub use boot::di::IProvider;
/// HTTP Request
pub use request::Request;
/// HTTP Response
pub use response::Response;

/// Module interface
pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
    fn providers(&self) -> Vec<Box<dyn IProvider>>;
    fn middlewares(&self) -> Vec<MiddlewareFunction>;
}

pub type MiddlewareFunction =
    Box<dyn Fn(Request, Response, NextFunction) -> Response + Send + Sync + UnwindSafe + 'static>;

/// Controller interface
pub trait IController {
    fn prefix(&self) -> String;
    fn routes(&self) -> Vec<Box<dyn IRoute + Send + 'static>>;
    fn middlewares(&self) -> Vec<MiddlewareFunction>;
}

/// Route interface
pub trait IRoute {
    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn handler(&self) -> Box<dyn IHandler + Send + 'static>;
}

/// Handler interface
pub trait IHandler: UnwindSafe {
    fn handle(&self, request: Request, response: Response) -> Response;
}

/// Next function type for middleware
pub type NextFunction = fn(Request, Response) -> Response;

/// Rupring Factory for creating server
#[derive(Debug, Clone)]
pub struct RupringFactory<T: IModule> {
    root_module: T,
}

impl<T: IModule + Clone + Copy + Sync + Send + 'static> RupringFactory<T> {
    /// It receives the root module object and creates a factory to run the server.
    pub fn create(module: T) -> Self {
        RupringFactory {
            root_module: module,
        }
    }

    /// It receives the port number and runs the server.
    pub async fn listen(self, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

        let result = boot::run_server(socket_addr, self.root_module).await;

        return result;
    }
}
