pub(crate) mod boot;
pub(crate) mod request;
pub(crate) mod response;

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
