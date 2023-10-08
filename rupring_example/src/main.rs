use rupring::{Controller, Get, Injectable, Module};

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

#[Injectable]
struct HomeService {}

#[tokio::main]
async fn main() {
    let root_module = HomeModule {};

    rupring::run_app().await;
}
