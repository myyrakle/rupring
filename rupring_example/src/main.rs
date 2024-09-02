use domains::root::module::RootModule;

pub(crate) mod domains;
pub(crate) mod middlewares;

fn main() {
    rupring::run(RootModule {})
}
