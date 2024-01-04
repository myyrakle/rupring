use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Default)]
pub struct SwaggerContext {
    pub openapi_json: Arc<RwLock<String>>,
}
