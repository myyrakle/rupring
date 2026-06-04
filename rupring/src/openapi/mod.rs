//! OpenAPI document generation and serving.
//!
//! This module owns the generated OpenAPI document and exposes `/openapi.json`.
//! Viewers such as Swagger UI can depend on this layer without owning document
//! generation themselves.

pub mod context;
pub mod controller;
pub mod json;
pub mod macros;
pub mod module;
pub mod routes;
pub mod swagger;

pub use json::*;

#[cfg(test)]
mod tests {
    use crate::core::route;
    use crate::openapi::swagger::module::SwaggerModule;
    use hyper::Method;

    #[test]
    fn swagger_viewer_lives_under_openapi_module() {
        let found_route = route::find_route(Box::new(SwaggerModule {}), "/docs", &Method::GET);

        assert!(found_route.is_some());
    }
}
