use domains::root::module::RootModule;

pub(crate) mod domains;
pub(crate) mod middlewares;

#[rupring::tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = rupring::RupringFactory::create(RootModule {});

    app.listen(3000).await?;

    Ok(())
}
