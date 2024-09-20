pub(crate) fn make_route_name(function_name: &str) -> String {
    let mut route_name = String::new();

    route_name.push_str(&format!("Route{}", function_name.chars().next().unwrap().to_uppercase().chain(function_name.chars().skip(1)))));

    return route_name;
}

pub(crate) fn make_handler_name(function_name: &str) -> String {
    let mut handler_name = String::new();

    handler_name.push_str(&format!("Handler{}", function_name.chars().next().unwrap().to_uppercase().chain(function_name.chars().skip(1)))));

    return handler_name;
}
