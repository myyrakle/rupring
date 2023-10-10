use rupring::{Controller, Get, IModule, Injectable, Module, Rupring};

#[Controller]
struct HomeController {
    home_service: HomeService,
}

impl HomeController {
    #[Get("/")]
    async fn index() {}
}

#[Module(
    controllers = [HomeController],
    providers = [HomeService],
    imports = []
)]
struct HomeModule {}

impl IModule for HomeModule {}

#[Injectable]
struct HomeService {}

#[tokio::main]
async fn main() {
    let root_module = HomeModule {};

    let app = Rupring::create(root_module);
    app.listen(8080).await;
}
