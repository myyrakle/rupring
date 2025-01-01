/*!
# Dependency Injection
- Dependency injection is a dependency management feature for complex systems with many dependencies between objects.
- Rupring provides flexible DI functions using traits and macros.

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
*/

use std::any::Any;
use std::panic::RefUnwindSafe;
use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct DIContext {
    pub containers: HashMap<TypeId, Box<dyn Any>>,
    wait_list: Vec<Box<dyn IProvider + 'static>>,
}

unsafe impl Send for DIContext {}
unsafe impl Sync for DIContext {}
impl RefUnwindSafe for DIContext {}

impl std::fmt::Debug for DIContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DIContext")
            .field("containers", &self.containers)
            .field("wait_list.len", &self.wait_list.len())
            .finish()
    }
}

impl DIContext {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(&mut self, value: Box<dyn Any>) {
        let type_id = (*value).type_id();
        if self.containers.contains_key(&type_id) {
            return;
        }

        self.containers.insert(type_id, value);
    }

    pub fn register_lazy<T: 'static>(&mut self, injectable: Box<dyn IProvider>) {
        self.wait_list.push(injectable);
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self.containers.get(&TypeId::of::<T>()) {
            Some(value) => value.downcast_ref::<T>(),
            None => None,
        }
    }
}

impl DIContext {
    fn import_from_modules(&mut self, root_module: Box<dyn crate::IModule>) {
        let providers = root_module.providers();

        for provider in providers {
            self.wait_list.push(provider);
        }

        let child_modules = root_module.child_modules();

        for child_module in child_modules {
            self.import_from_modules(child_module);
        }
    }

    pub fn initialize(&mut self, root_module: Box<dyn crate::IModule>) {
        self.import_from_modules(root_module);

        while !self.wait_list.is_empty() {
            let mut has_ready_provider = false;

            for (i, provider) in self.wait_list.iter().enumerate() {
                let dependencies = provider.dependencies();

                let mut is_ready = true;

                for dependency in dependencies {
                    if !self.containers.contains_key(&dependency) {
                        is_ready = false;
                        break;
                    }
                }

                if is_ready {
                    let provided_value = provider.provide(self);
                    self.register(provided_value);
                    self.wait_list.remove(i);
                    has_ready_provider = true;

                    break;
                }
            }

            if !has_ready_provider {
                panic!("No provider is ready");
            }
        }
    }
}

pub trait IProvider {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn provide(&self, di_context: &DIContext) -> Box<dyn Any>;
}
