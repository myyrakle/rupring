// #![allow(clippy::single_match)]
// #![allow(clippy::to_string_trait_impl)]
// #![allow(clippy::map_entry)]
// #![allow(clippy::map_flatten)]

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

fn main() {
    rupring::run(RootModule {})
}
```

# Request
- rupring defines HTTP Request through [crate::request::Request] type and provides convenient request processing using macros.
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
- Please refer to the corresponding [document](crate::request) for more details.

# Response
- rupring defines HTTP Response through [crate::response::Response] type and provides convenient response processing using macros.
```rust
#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("Hello, World!".to_string())
}
```
- Please refer to the corresponding [document](crate::response) for more details.

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
- Rupring provides powerful DI features through macro and runtime support.
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
- Please refer to the corresponding [document](crate::di) for more details.

# Swagger
- When rupring starts the server, it automatically serves swagger documents to the `/docs` path.
- Please refer to the corresponding [document](crate::swagger) for more details.

# Application Properties
- rupring provides various execution options through a special configuration file called application.properties.
- Please refer to the corresponding [document](crate::application_properties) for more details.

# AWS Lambda
- rupring provides the option to run on AWS Lambda.
- Supported Lambda Runtimes
    1. Amazon Linux 2
    2. Amazon Linux 2023

## How to use
1. Enable the "aws-lambda" feature flag.
```ignore
rupring={ version = "0.12.0", features=["aws-lambda"] }
```

2. Use the `rupring::run_on_aws_lambda` function instead of `rupring::run`.
```rust,ignore
fn main() {
    rupring::run_on_aws_lambda(RootModule {})
}
```

3. Compile and create an executable file. (x86_64-unknown-linux-musl)
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

3. Zip the executable file and upload it to the AWS console.
- The name of the executable file must be `bootstrap`.
```bash
zip -j bootstrap.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```

4. ...and upload it as a file to the AWS console
*/

pub(crate) mod core;
pub(crate) mod utils;
pub use core::boot::run;

#[cfg(feature = "aws-lambda")]
pub use core::boot::run_on_aws_lambda;

pub mod di;

/// header constants
pub mod header;
/// http protocol related module
pub mod http;
mod logger;
/// HTTP request module
pub mod request;
/// HTTP response module
pub mod response;
/// swagger module
pub mod swagger;

use std::panic::UnwindSafe;

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

/// Application Properties
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
    pub fn listen(self) -> anyhow::Result<()> {
        use tokio::runtime::Builder;

        let mut runtime_builder = Builder::new_multi_thread();

        runtime_builder.enable_all();

        if let Some(thread_limit) = self.application_properties.server.thread_limit {
            runtime_builder.worker_threads(thread_limit);
        }

        let runtime = runtime_builder.build()?;

        runtime.block_on(async {
            core::run_server(self.application_properties, self.root_module).await
        })
    }

    #[cfg(feature = "aws-lambda")]
    pub fn listen_on_aws_lambda(self) -> anyhow::Result<()> {
        use tokio::runtime::Builder;

        let mut runtime_builder = Builder::new_multi_thread();

        runtime_builder.enable_all();

        if let Some(thread_limit) = self.application_properties.server.thread_limit {
            runtime_builder.worker_threads(thread_limit);
        }

        let runtime = runtime_builder.build()?;

        runtime.block_on(async {
            core::run_server_on_aws_lambda(self.application_properties, self.root_module).await
        })
    }
}

/// RupringDto derive macro
pub use rupring_macro::RupringDto;

#[cfg(test)]
mod test_proc_macro;

pub use anyhow;
pub use anyhow::anyhow as error;
pub use anyhow::Result;

//pub use serde;
//pub use serde::{Deserialize, Serialize};
pub use serde_json;

pub use tokio;
