use std::sync::{Arc, RwLock};

use crate as rupring;
use crate::IModule;

use super::{
    json::{SwaggerPath, SwaggerSchema},
    SwaggerTags,
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
            let normalized_path = crate::boot::route::normalize_path(prefix.clone(), route.path());
            let operation = route.swagger();

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
