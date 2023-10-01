use rupring_macro::{Controller, Injectable, Module};

#[Controller]
struct HomeController {}

#[Module]
struct HomeModule {}

#[Injectable]
struct HomeService {}

fn main() {
    let controller = HomeController {};

    rupring::run_app();
}
