use std::{collections::HashMap, sync::Arc};

use crate::Method;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, Vec<String>>,
    pub path_parameters: HashMap<String, String>,
    pub di_context: Arc<crate::DIContext>,
}
