use crate::IModule;

/** shortcut to run the application

```rust,ignore
use domains::root::module::RootModule;

pub(crate) mod domains;
pub(crate) mod middlewares;

fn main() {
    rupring::run(RootModule {})
}
```
*/
#[tokio::main]
pub async fn run<T>(root_module: T)
where
    T: IModule + Clone + Copy + Sync + Send + 'static,
{
    let app = crate::RupringFactory::create(root_module);

    let port = app.application_properties.server.port;

    app.listen(port).await.unwrap();
}
