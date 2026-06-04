use super::context::InjectOpenApiContext;
use super::controller::OpenApiController;
use crate as rupring;

#[derive(Debug, Clone)]
#[rupring_macro::Module(
    controllers = OpenApiController{},
    providers = [InjectOpenApiContext{}]
)]
pub struct OpenApiModule {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::route;
    use hyper::Method;

    #[test]
    fn openapi_module_exposes_openapi_json_endpoint() {
        let found_route =
            route::find_route(Box::new(OpenApiModule {}), "/openapi.json", &Method::GET);

        assert!(found_route.is_some());
    }
}
