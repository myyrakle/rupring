use std::sync::{Arc, RwLock};

use crate::IModule;
use crate::{self as rupring};

use super::{
    json::{SwaggerPath, SwaggerSchema},
    SwaggerTags,
};
use super::{
    SwaggerParameter, SwaggerParameterCategory, SwaggerReference, SwaggerResponse,
    SwaggerTypeOrReference,
};

#[derive(Debug, Clone, Default)]
pub struct SwaggerContext {
    pub openapi_json: Arc<RwLock<String>>,
}

impl SwaggerContext {
    pub fn initialize_from_module(&self, module: impl IModule + Clone + 'static) {
        let mut swagger = SwaggerSchema::default();
        swagger.tags = unsafe { SWAGGER_TAGS.0.clone() };

        generate_swagger(&mut swagger, Box::new(module));

        let mut openapi_json = self.openapi_json.write().unwrap();
        *openapi_json = serde_json::to_string(&swagger).unwrap();
    }
}

#[rupring::Component(name=InjectSwaggerContext)]
pub fn inject_swagger_context() -> SwaggerContext {
    SwaggerContext::default()
}

fn to_string(method: hyper::Method) -> String {
    match method {
        hyper::Method::GET => "get".to_string(),
        hyper::Method::POST => "post".to_string(),
        hyper::Method::PUT => "put".to_string(),
        hyper::Method::DELETE => "delete".to_string(),
        hyper::Method::HEAD => "head".to_string(),
        hyper::Method::OPTIONS => "options".to_string(),
        hyper::Method::CONNECT => "connect".to_string(),
        hyper::Method::PATCH => "patch".to_string(),
        hyper::Method::TRACE => "trace".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

static mut SWAGGER_TAGS: SwaggerTags = SwaggerTags::new();

fn generate_swagger(swagger: &mut SwaggerSchema, root_module: Box<dyn crate::IModule>) {
    for controller in root_module.controllers() {
        let prefix = controller.prefix();

        for route in controller.routes() {
            let normalized_path = crate::core::route::normalize_path(prefix.clone(), route.path());
            let normalized_path = swaggerize_url(normalized_path.as_str());
            let mut operation = route.swagger();

            for security in route.swagger_security_info() {
                operation.security.push(security);
            }

            let request_info = route.swagger_request_info();

            if let Some(swagger_request_body) = request_info {
                operation.parameters.push(SwaggerParameter {
                    name: swagger_request_body
                        .definition_name
                        .split("::")
                        .last()
                        .unwrap_or("Request Body")
                        .to_string(),
                    in_: SwaggerParameterCategory::Body,
                    description: "Request Body".to_string(),
                    required: true,
                    schema: Some(SwaggerTypeOrReference::Reference(SwaggerReference {
                        reference: "#/definitions/".to_string()
                            + swagger_request_body.definition_name.as_str(),
                    })),
                    type_: None,
                });

                swagger.definitions.insert(
                    swagger_request_body.definition_name.clone(),
                    swagger_request_body.definition_value,
                );

                for dependency in swagger_request_body.dependencies {
                    swagger.definitions.insert(
                        dependency.definition_name.clone(),
                        dependency.definition_value,
                    );
                }

                for swagger_parameter in swagger_request_body.path_parameters {
                    operation.parameters.push(swagger_parameter);
                }

                for swagger_parameter in swagger_request_body.query_parameters {
                    operation.parameters.push(swagger_parameter);
                }
            }

            let response_info = route.swagger_response_info();

            if let Some(swagger_response_body) = response_info {
                swagger.definitions.insert(
                    swagger_response_body.definition_name.clone(),
                    swagger_response_body.definition_value,
                );

                operation.responses.insert(
                    "200".to_string(),
                    SwaggerResponse {
                        description: "OK".to_string(),
                        schema: Some(SwaggerReference {
                            reference: "#/definitions/".to_string()
                                + swagger_response_body.definition_name.as_str(),
                        }),
                    },
                );

                for dependency in swagger_response_body.dependencies {
                    swagger.definitions.insert(
                        dependency.definition_name.clone(),
                        dependency.definition_value,
                    );
                }
            }

            // TODO: 추후에는 swagger ignore 속성을 추가해서 그걸로 처리
            match normalized_path.as_str() {
                "/docs/swagger.json"
                | "/docs"
                | "/docs/index.css"
                | "/docs/favicon-16x16.png"
                | "/docs/favicon-32x32.png"
                | "/docs/swagger-initializer.js"
                | "/docs/swagger-ui.css"
                | "/docs/swagger-ui-standalone-preset.js"
                | "/docs/swagger-ui-bundle.js" => continue,
                _ => {}
            }

            let method = to_string(route.method());

            if let Some(path) = swagger.paths.get_mut(&normalized_path) {
                if let Some(_) = path.get(&method) {
                    continue;
                }

                path.insert(method, operation);
                continue;
            }

            let mut path = SwaggerPath::default();

            path.insert(method, operation);
            swagger.paths.insert(normalized_path, path);
        }
    }

    for child_module in root_module.child_modules() {
        generate_swagger(swagger, child_module);
    }
}

// /:id/do-something -> /{id}/do-something
fn swaggerize_url(url: &str) -> String {
    let mut result = String::new();

    let mut is_segment_start = true;
    let mut in_path_param = false;

    for c in url.chars() {
        match c {
            '/' => {
                if in_path_param {
                    result.push('}');
                    in_path_param = false;
                }

                is_segment_start = true;

                result.push(c);

                continue;
            }
            ':' if is_segment_start => {
                is_segment_start = false;
                in_path_param = true;
                result.push('{');
                continue;
            }
            _ => {
                is_segment_start = false;
                result.push(c);
            }
        }
    }

    if in_path_param {
        result.push('}');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swaggerize_url() {
        assert_eq!(swaggerize_url("/users/:id"), "/users/{id}");
        assert_eq!(swaggerize_url("users/:id"), "users/{id}");
        assert_eq!(
            swaggerize_url("/users/:id/do-something"),
            "/users/{id}/do-something"
        );
        assert_eq!(
            swaggerize_url("/users/:id/do-something/:id2"),
            "/users/{id}/do-something/{id2}"
        );
        assert_eq!(
            swaggerize_url("/users/:id/do-something/:id2/"),
            "/users/{id}/do-something/{id2}/"
        );
    }
}
