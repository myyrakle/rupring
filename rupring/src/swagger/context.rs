use std::borrow::Borrow;
use std::sync::Mutex;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::IModule;

use super::json::{SwaggerPath, SwaggerSchema};

#[derive(Debug, Clone, Default)]
pub struct SwaggerContext {
    pub openapi_json: Arc<RwLock<String>>,
}

impl SwaggerContext {
    pub fn initialize_from_module(&self, module: impl IModule + Clone + 'static) {
        let mut swagger = SwaggerSchema::default();

        generate_swagger(&mut swagger, Box::new(module));

        let mut openapi_json = self.openapi_json.write().unwrap();
        *openapi_json = serde_json::to_string(&swagger).unwrap();
    }
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

fn generate_swagger(swagger: &mut SwaggerSchema, root_module: Box<dyn crate::IModule>) {
    for controller in root_module.controllers() {
        let prefix = controller.prefix();

        for route in controller.routes() {
            let normalized_path = crate::boot::route::normalize_path(prefix.clone(), route.path());

            let method = to_string(route.method());

            swagger.paths.contains_key(&normalized_path)
        }
    }

    for child_module in root_module.child_modules() {
        generate_swagger(swagger, child_module);
    }
}
