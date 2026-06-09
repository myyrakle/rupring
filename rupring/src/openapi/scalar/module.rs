use super::controller::ScalarController;
use crate as rupring;
use crate::openapi::module::OpenApiModule;

#[derive(Debug, Clone)]
#[rupring_macro::Module(
    controllers = ScalarController{},
    modules = [OpenApiModule{}],
    providers = []
)]
pub struct ScalarModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::route;
    use hyper::Method;

    #[test]
    fn scalar_module_exposes_canonical_openapi_json_endpoint() {
        let found_route =
            route::find_route(Box::new(ScalarModule {}), "/openapi.json", &Method::GET);

        assert!(found_route.is_some());
    }
}
