use domains::root::module::RootModule;

pub(crate) mod domains;
pub(crate) mod middlewares;

struct Foo {}

trait Some {
    fn some() -> i32;
}

impl Some for Foo {
    fn some() -> i32 {
        1
    }
}

impl Some for Option<Foo> {
    fn some() -> i32 {
        1
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = rupring::RupringFactory::create(RootModule {});

    app.listen(3000).await?;

    Ok(())
}
