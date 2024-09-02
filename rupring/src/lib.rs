/*! # Get Started
There is only one dependency.
```bash
cargo add rupring
```

And you can write your server like this:
```rust,ignore
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

# Request
You can access any value provided in an HTTP Request through the Request parameter.

```rust
#[rupring::Get(path = /:id)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    let method = request.method;
    assert_eq!(method, rupring::Method::GET);

    let path = request.path;
    assert_eq!(path, "/");

    let body = request.body;
    assert_eq!(body, "");

    let headers = request.headers;
    let content_type = headers.get("content-type").unwrap();
    assert_eq!(content_type, "text/plain");

    let id = request.path_parameters["id"].clone();
    assert_eq!(id, "123");

    let query = request.query_parameters["query"].clone();
    assert_eq!(query, vec!["asdf".to_string()]);

    //...

    response
}
```

## Request: Path Param

For path parameters, auto binding is provided through annotation.

The annotation name can be one of `Path`, `path`, or `PathVariable`.
```rust
#[rupring::Get(path = /echo/:id)]
pub fn echo(
    #[PathVariable="id"] id: i32
) -> rupring::Response {
    println!("id: {}", id);

    rupring::Response::new().text(request.body)
}
```

If the Path Param is optional, just wrap the type in `Option`.
```rust
#[rupring::Get(path = /echo/:id)]
pub fn echo(
    #[PathVariable="id"] id: Option<i32>
) -> rupring::Response {
    // ...

    rupring::Response::new().text("OK".to_string())
}
```

If you need Swagger documentation for the Path Param, you should add the `Description` annotation.
`Description` can also be used as `Desc`, `desc`, etc.
```rust
#[rupring::Get(path = /echo/:id)]
pub fn echo(
    #[path="id"] #[desc="asdf"] id: i32
) -> rupring::Response {
    println!("id: {}", id);

    rupring::Response::new().text(request.body)
}
```

If you want to define a custom type for PathParam, you can implement the ParamStringDeserializer trait.
```rust
struct SomeCustomType {}

impl rupring::ParamStringDeserializer<SomeCustomType> for rupring::ParamString {
    type Error = ();

    fn deserialize(&self) -> Result<SomeCustomType, Self::Error> {
        //...
        Ok(SomeCustomType {})
    }
}
```


# Response

You can create a response like this:
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("Hello, World!".to_string())
}
```

You can also return a json value like this:
```rust
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
```rust
#[rupring::Get(path = /asdf)]
pub fn not_found(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("not found".to_string()).status(404)
}
```

You can set the header like this:
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text("Hello, World!".to_string())
        .header("content-type", "text/plain".to_string())
}
```

If you want, you can receive it as a parameter instead of creating the response directly.
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request, response: rupring::Response) -> rupring::Response {
    response
        .text("Hello, World!".to_string())
        .header("content-type", "text/plain".to_string())
}
```
This is especially useful when you need to inherit and use Response through middleware.

If you want to redirect, you can use Response’s redirect method.
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().redirect("/hello")
}
```
This method automatically sets status to 302 unless you set it to 300-308.

# Middleware
rupring provides middleware features for common logic processing.

If you want to log requests for all APIs that exist in a module, you can apply middleware in the form below.

First, define a middleware function.
```rust
pub fn logger_middleware(
    request: rupring::Request,
    response: rupring::Response,
    next: rupring::NextFunction,
) -> rupring::Response {
    println!(
        "Request: {} {}",
        request.method.to_string(),
        request.path.to_string()
    );

    next(request, response)
}
```
The above function only records logs and forwards them to the next middleware or route function.
If you want to return a response immediately without forwarding, just return the response without calling the next function.


And you can register the middleware function just defined in the module or controller unit.
```rust
pub fn logger_middleware(
    request: rupring::Request,
    response: rupring::Response,
    next: rupring::NextFunction,
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
    controllers=[RootController{}],
    modules=[UserModule{}],
    providers=[],
    middlewares=[logger_middleware]
)]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[])]
pub struct RootController {}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[UserController{}],
    providers=[],
    middlewares=[]
)]
pub struct UserModule {}

// or Controller
#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[], middlewares=[logger_middleware])]
pub struct UserController {}
```
Middleware registered in a module is recursively applied to the routes of controllers registered in that module and to child modules.
On the other hand, middleware registered in a controller applies only to the routes of that controller.

The priorities in which middleware is applied are as follows:

1. Middleware of the same unit is executed in the order defined in the array.
2. If module middleware and controller middleware exist at the same time, module middleware is executed first.
3. If the parent module's middleware and the child module's middleware exist at the same time, the parent module middleware is executed first.


# Dependency Injection

rupring provides DI feature.

If you want to implement and DI a Provider that contains simple functionality, you can do it like this:
First, define the Provider.
```rust
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
```rust
use std::any::Any;

#[derive(Debug, Clone, Default)]
pub struct HomeService {}

impl rupring::IProvider for HomeService {
    fn provide(&self, _di_context: &rupring::DIContext) -> Box<dyn Any> {
        Box::new(HomeService {})
    }
}

#[derive(Debug, Clone, Copy)]
#[rupring::Controller(prefix=/, routes=[])]
pub struct HomeController {}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[], providers=[HomeService::default()])]
pub struct RootModule {}
```

And, you can use it by getting it from the router through the request object.
```rust
pub struct HomeService {}

impl HomeService {
    pub fn hello(&self) -> String {
        "hello!!".to_string()
    }
}

#[rupring::Get(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    let home_service = request.get_provider::<HomeService>().unwrap();

    rupring::Response::new().text(home_service.hello())
}
```

If a provider requires another provider, you must specify the dependency cycle as follows:
```rust
use std::any::TypeId;

#[derive(Debug, Clone, Default)]
pub struct HomeRepository {}

pub struct HomeService {
    home_repository: HomeRepository,
}

impl HomeService {
    pub fn hello(&self) -> String {
        "hello!!".to_string()
    }
}

impl rupring::IProvider for HomeService {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![TypeId::of::<HomeRepository>()]
    }

    fn provide(&self, di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {
        Box::new(HomeService {
            home_repository: di_context.get::<HomeRepository>().map(|e|e.to_owned()).unwrap(),
        })
    }
}
```

If you need mutables within the provider, you must ensure thread safety through Mutex or Atomic as follows:
```rust
use std::sync::{Arc, Mutex};

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

```rust
use std::any::TypeId;

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

Additionally, shortcuts are provided for defining DI components.
For example, the code below automatically creates an IProvider object "inject_counter_service" that can be passed to modules.
```rust
#[derive(Debug, Clone, Default)]
pub struct SomethingRepository {}

#[derive(Debug, Clone, Default)]
pub struct CounterService {
    something: SomethingRepository,
}

impl CounterService {
    pub fn new(something: SomethingRepository) -> Self {
        CounterService { something }
    }
}

#[rupring::Injectable]
fn inject_counter_service(something: SomethingRepository) -> CounterService {
    CounterService::new(something)
}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[/*...*/],
    modules=[/*...*/],
    providers=[inject_counter_service{}],
    middlewares=[]
)]
pub struct RootModule {}
```
It automatically receives DI based on parameters.

The injectable annotation can also be explicitly named.
```rust
#[derive(Debug, Clone, Default)]
pub struct SomethingRepository {}

#[derive(Debug, Clone, Default)]
pub struct CounterService {
    something: SomethingRepository,
}

impl CounterService {
    pub fn new(something: SomethingRepository) -> Self {
        CounterService { something }
    }
}

#[rupring::Injectable(CounterServiceFactory)] // or #[rupring::Injectable(name=CounterServiceFactory)]
fn inject_counter_service(something: SomethingRepository) -> CounterService {
    CounterService::new(something)
}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[/*...*/],
    modules=[/*...*/],
    providers=[CounterServiceFactory{}],
    middlewares=[]
)]
pub struct RootModule {}
```

# Swagger
When rupring starts the server, it automatically serves swagger documents to the `/docs` path.

Additional annotations such as `summary`, `description`, and `tags` are provided for swagger documentation.
```rust
#[rupring::Get(path = /echo/:id)]
#[summary = "echo API"]
#[description = "It's echo API"]
#[tags = ["echo"]]
pub fn echo(
    #[path="id"] #[description="just integer id"] id: Option<i32>
) -> rupring::Response {
    //...

    rupring::Response::new().text("OK".to_string())
}
```

Using the RupringDoc derive macro, you can perform document definition for Request Parameter.
```rust
use rupring::RupringDoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct CreateUserRequest {
    #[desc = "user name"]
    #[example = "foobar"]
    pub username: String,

    pub email: String,

    #[desc = "user password"]
    #[example = "q1w2e3r4"]
    pub password: String,
}
```
### RupringDoc attribute Details
1. `#[desc = ""]` or `#[description = ""]`: Description of the field.
2. `#[example = ""]`: Example value of the field.
3. `#[name = "id"]`: If the field name is different from the variable name, you can add this annotation.
4. `#[required]`: If the field is required, you can add this annotation.
5. `#[path_param = "id"]` or `#[param = "id"]`: If the field is a path parameter, you can add this annotation.
6. `#[query = "query"]`: If the field is a query parameter, you can add this annotation.
7. `#[ignore]`: If you want to ignore the field, you can add this annotation.

Then, you can specify request information in the API through the params attribute as follows.
```rust
use rupring::RupringDoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct CreateUserRequest {
    #[desc = "user name"]
    #[example = "foobar"]
    pub username: String,

    pub email: String,

    #[desc = "user password"]
    #[example = "q1w2e3r4"]
    pub password: String,
}

#[rupring::Post(path = /users)]
#[tags = [user]]
#[summary = "user create"]
#[params = CreateUserRequest]
pub fn create_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    // ...

    rupring::Response::new().text("OK".to_string())
}
```

Response documentation can also be defined through the RupringDoc macro and response attribute.
```rust
use rupring::RupringDoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[rupring::Get(path = /users/:id)]
#[tags = [user]]
#[summary = "find user"]
#[response = GetUserResponse]
pub fn get_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    return rupring::Response::new().text("OK".to_string());
}
```

If you want to activate BearerAuth for the API, activate the auth attribute as follows. (The default is BearerAuth.
```rust
use rupring::RupringDoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDoc)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[rupring::Get(path = /users/:id)]
#[tags = [user]]
#[summary = "find user"]
#[response = GetUserResponse]
#[auth = BearerAuth]
pub fn get_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    return rupring::Response::new().text("OK".to_string());
}
```
*/

pub(crate) mod core;
pub use core::boot::run;
pub(crate) mod di;

/// header constants
pub mod header;
mod logger;
/// MEME type constants
pub mod meme;
/// HTTP request module
pub mod request;
/// HTTP response module
pub mod response;
/// swagger module
pub mod swagger;

use std::panic::UnwindSafe;
use std::str::FromStr;

use application_properties::load_application_properties_from_all;
use application_properties::ApplicationProperties;
/**  Controller Annotation
```rust
#[rupring::Get(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text("Hello, World!".to_string())
}

#[rupring::Get(path = /echo)]
pub fn echo(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text(request.body)
}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, echo])]
pub struct HomeController {}
```
*/
pub use rupring_macro::Controller;

/** Module Annotation
```rust
#[derive(Debug, Clone)]
#[rupring::Module(
    controllers=[/*HomeController{}*/],
    modules=[],
    providers=[/*HomeService::default(), HomeRepository::default(), UserService::default(), CounterService::default()*/]
)]
pub struct RootModule {}
```
 */
pub use rupring_macro::Module;

/** This is a shortcut annotation for creating an IProvider object.

```rust
use std::sync::{Arc, Mutex};

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
}

#[rupring_macro::Injectable(CounterServiceFactory)]
fn inject_counter_service() -> CounterService {
   CounterService::new()
}

#[derive(Debug, Clone, Copy)]
#[rupring::Module(
    controllers=[/*...*/],
    modules=[/*...*/],
    providers=[CounterServiceFactory{}],
    middlewares=[]
)]
pub struct RootModule {}
```
*/
pub use rupring_macro::Injectable;

/// This is an alias for [Injectable].
pub use rupring_macro::Bean;

/// This is an alias for [Injectable].
pub use rupring_macro::Component;

/// This is an alias for [Injectable].
pub use rupring_macro::Service;

/// This is an alias for [Injectable].
pub use rupring_macro::Repository;

/** Get Route Annotation
```rust
#[rupring::Get(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text("Hello, World!".to_string())
}
*/
pub use rupring_macro::Get;

/// This is an alias for [Get].
pub use rupring_macro::GetMapping;

/** Post Route Annotation
```rust
#[rupring::Post(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text("Hello, World!".to_string())
}
```
*/
pub use rupring_macro::Post;

/// This is an alias for [Post].
pub use rupring_macro::PostMapping;

/** Patch Route Annotation
```rust
#[rupring::Patch(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text("Hello, World!".to_string())
}
```
*/
pub use rupring_macro::Patch;

/// This is an alias for [Patch].
pub use rupring_macro::PatchMapping;

/** Put Route Annotation
```rust
#[rupring::Put(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text("Hello, World!".to_string())
}
```
*/
pub use rupring_macro::Put;

/// This is an alias for [Put].
pub use rupring_macro::PutMapping;

/** Delete Route Annotation
```rust
#[rupring::Delete(path = /)]
pub fn hello(request: rupring::Request) -> rupring::Response {
    // ...
    rupring::Response::new().text("Hello, World!".to_string())
}
```
*/
pub use rupring_macro::Delete;

/// This is an alias for [Delete].
pub use rupring_macro::DeleteMapping;

/// HTTP method (from hyper crate)
pub type Method = hyper::Method;

/// HTTP Header Name (from hyper crate)
pub type HeaderName = hyper::header::HeaderName;

/// Dependency Injection Context for entire life cycle
pub use di::DIContext;
/// Dependency Injection Provider
pub use di::IProvider;
/// String wrapper type for ParamStringDeserializer.
pub use request::ParamString;
/// ParamStringDeserializer trait
pub use request::ParamStringDeserializer;
/// HTTP Request
pub use request::Request;
/// HTTP Response
pub use response::Response;
use swagger::json::SwaggerOperation;
use swagger::macros::SwaggerRequestBody;
use swagger::SwaggerSecurity;

pub mod application_properties;

/// Module interface
pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
    fn providers(&self) -> Vec<Box<dyn IProvider>>;
    fn middlewares(&self) -> Vec<MiddlewareFunction>;
}

/// Middleware function type
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

    fn swagger(&self) -> SwaggerOperation {
        Default::default()
    }

    fn swagger_request_info(&self) -> Option<SwaggerRequestBody> {
        None
    }

    fn swagger_response_info(&self) -> Option<SwaggerRequestBody> {
        None
    }

    fn swagger_security_info(&self) -> Vec<SwaggerSecurity> {
        vec![]
    }
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
    pub application_properties: ApplicationProperties,
}

impl<T: IModule + Clone + Copy + Sync + Send + 'static> RupringFactory<T> {
    /// It receives the root module object and creates a factory to run the server.
    pub fn create(module: T) -> Self {
        RupringFactory {
            root_module: module,
            application_properties: load_application_properties_from_all(),
        }
    }

    /// It receives the port number and runs the server.
    pub async fn listen(self, port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use std::net::{IpAddr, SocketAddr};

        let host = self.application_properties.server.address.clone();

        let ip = IpAddr::from_str(host.as_str())?;

        let socket_addr = SocketAddr::new(ip, port);

        let result = core::run_server(socket_addr, self.root_module).await;

        return result;
    }
}

pub use rupring_macro::RupringDoc;

#[cfg(test)]
mod test_proc_macro;

pub use anyhow;
pub use anyhow::anyhow as error;
pub use anyhow::Result;

//pub use serde;
//pub use serde::{Deserialize, Serialize};
pub use serde_json;

pub use tokio;
