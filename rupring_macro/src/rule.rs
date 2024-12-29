pub(crate) fn make_route_name(function_name: &str) -> String {
    let mut route_name = String::new();

    route_name.push_str(&format!("Route_{}", function_name));

    route_name
}

pub(crate) fn make_handler_name(function_name: &str) -> String {
    let mut handler_name = String::new();

    handler_name.push_str(&format!("Handler_{}", function_name));

    handler_name
}
