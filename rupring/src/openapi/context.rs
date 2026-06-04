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
pub struct OpenApiContext {
    pub openapi_json: Arc<RwLock<String>>,
}

impl OpenApiContext {
    pub fn initialize_from_module(&self, module: impl IModule + Clone + 'static) {
        let mut swagger = SwaggerSchema {
            tags: { OPENAPI_TAGS.0.clone() },
            ..Default::default()
        };

        generate_openapi(&mut swagger, Box::new(module));

        let mut openapi_json = self.openapi_json.write().unwrap();
        *openapi_json = serde_json::to_string(&swagger).unwrap();
    }
}

#[rupring::Component(name=InjectOpenApiContext)]
pub fn inject_openapi_context() -> OpenApiContext {
    OpenApiContext::default()
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

static OPENAPI_TAGS: SwaggerTags = SwaggerTags::new();

fn should_skip_documentation_path(path: &str) -> bool {
    matches!(
        path,
        "/openapi.json"
            | "/docs/swagger.json"
            | "/docs"
            | "/docs/favicon-16x16.png"
            | "/docs/favicon-32x32.png"
            | "/docs/swagger-initializer.js"
            | "/docs/swagger-ui.css"
            | "/docs/swagger-ui-standalone-preset.js"
            | "/docs/swagger-ui-bundle.js"
    )
}

fn generate_openapi(swagger: &mut SwaggerSchema, root_module: Box<dyn crate::IModule>) {
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
                let has_body_properties =
                    !swagger_request_body.definition_value.properties.is_empty();

                if has_body_properties {
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

            if should_skip_documentation_path(&normalized_path) {
                continue;
            }

            let method = to_string(route.method());

            if let Some(path) = swagger.paths.get_mut(&normalized_path) {
                if path.get(&method).is_some() {
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
        generate_openapi(swagger, child_module);
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
    use crate::openapi::json::SwaggerDefinitionObject;
    use crate::{IController, IHandler, IModule, IProvider, IRoute, Method, Request, Response};
    use std::collections::HashMap;

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

    #[test]
    fn get_route_with_query_only_request_info_does_not_generate_body_parameter() {
        #[derive(Clone)]
        struct TestModule;

        impl IModule for TestModule {
            fn child_modules(&self) -> Vec<Box<dyn IModule>> {
                vec![]
            }

            fn controllers(&self) -> Vec<Box<dyn IController>> {
                vec![Box::new(TestController)]
            }

            fn providers(&self) -> Vec<Box<dyn IProvider>> {
                vec![]
            }

            fn middlewares(&self) -> Vec<crate::MiddlewareFunction> {
                vec![]
            }
        }

        struct TestController;

        impl IController for TestController {
            fn prefix(&self) -> String {
                "/".to_string()
            }

            fn routes(&self) -> Vec<Box<dyn IRoute + Send + 'static>> {
                vec![Box::new(TestRoute)]
            }

            fn middlewares(&self) -> Vec<crate::MiddlewareFunction> {
                vec![]
            }
        }

        struct TestRoute;

        impl IRoute for TestRoute {
            fn method(&self) -> Method {
                Method::GET
            }

            fn path(&self) -> String {
                "/users".to_string()
            }

            fn handler(&self) -> Box<dyn IHandler + Send + 'static> {
                Box::new(TestHandler)
            }

            fn swagger_request_info(&self) -> Option<crate::openapi::macros::SwaggerRequestBody> {
                Some(crate::openapi::macros::SwaggerRequestBody {
                    definition_name: "ListUsersRequest".to_string(),
                    definition_value: SwaggerDefinitionObject {
                        type_: "object".to_string(),
                        properties: HashMap::new(),
                        required: vec![],
                        path_parameters: vec![],
                        query_parameters: vec![
                            SwaggerParameter {
                                name: "offset".to_string(),
                                in_: SwaggerParameterCategory::Query,
                                description: "".to_string(),
                                required: false,
                                type_: Some("number".to_string()),
                                schema: None,
                            },
                            SwaggerParameter {
                                name: "limit".to_string(),
                                in_: SwaggerParameterCategory::Query,
                                description: "".to_string(),
                                required: false,
                                type_: Some("number".to_string()),
                                schema: None,
                            },
                        ],
                    },
                    dependencies: vec![],
                    path_parameters: vec![],
                    query_parameters: vec![
                        SwaggerParameter {
                            name: "offset".to_string(),
                            in_: SwaggerParameterCategory::Query,
                            description: "".to_string(),
                            required: false,
                            type_: Some("number".to_string()),
                            schema: None,
                        },
                        SwaggerParameter {
                            name: "limit".to_string(),
                            in_: SwaggerParameterCategory::Query,
                            description: "".to_string(),
                            required: false,
                            type_: Some("number".to_string()),
                            schema: None,
                        },
                    ],
                })
            }
        }

        struct TestHandler;

        impl IHandler for TestHandler {
            fn handle(&self, _request: Request, response: Response) -> Response {
                response
            }
        }

        let openapi_context = OpenApiContext::default();
        openapi_context.initialize_from_module(TestModule);
        let json = openapi_context.openapi_json.read().unwrap().to_owned();
        let schema: serde_json::Value = serde_json::from_str(&json).unwrap();
        let parameters = schema["paths"]["/users"]["get"]["parameters"]
            .as_array()
            .unwrap();

        assert!(parameters
            .iter()
            .all(|parameter| parameter["in"] != serde_json::json!("body")));
        assert_eq!(parameters.len(), 2);
    }
}
