use rupring::{Controller, Injectable, Module};

#[Controller]
struct HomeController {}

#[Module]
struct HomeModule {}

#[Injectable]
struct HomeService {}

#[tokio::main]
async fn main() {
    let controller = HomeController {};

    rupring::run_app().await;
}
