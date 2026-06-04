use super::controller::SwaggerController;
use crate as rupring;
use crate::openapi::module::OpenApiModule;

#[derive(Debug, Clone)]
#[rupring_macro::Module(
    controllers = SwaggerController{},
    modules = [OpenApiModule{}],
    providers = []
)]
pub struct SwaggerModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::route;
    use hyper::Method;

    #[test]
    fn swagger_module_exposes_canonical_openapi_json_endpoint() {
        let found_route =
            route::find_route(Box::new(SwaggerModule {}), "/openapi.json", &Method::GET);

        assert!(found_route.is_some());
    }
}
