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
pub fn run<T>(root_module: T)
where
    T: IModule + Clone + Copy + Sync + Send + 'static,
{
    let app = crate::RupringFactory::create(root_module);

    app.listen().unwrap();
}

#[cfg(feature = "aws-lambda")]
pub fn run_on_aws_lambda<T>(root_module: T)
where
    T: IModule + Clone + Copy + Sync + Send + 'static,
{
    let app = crate::RupringFactory::create(root_module);

    if let Err(error) = app.listen_on_aws_lambda() {
        println!("Unhandled Error: {:?}", error);
    }
}
