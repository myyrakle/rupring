use std::{collections::HashMap, panic::UnwindSafe, sync::Arc};

use crate::Method;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, Vec<String>>,
    pub path_parameters: HashMap<String, String>,
    pub(crate) di_context: Arc<crate::DIContext>,
}

impl UnwindSafe for Request {}

impl Request {
    pub fn get_provider<T: 'static>(&self) -> Option<&T> {
        return self.di_context.get::<T>();
    }
}
