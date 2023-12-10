use std::collections::HashMap;

use crate::Method;

#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, Vec<String>>,
    pub path_parameters: HashMap<String, String>,
}
