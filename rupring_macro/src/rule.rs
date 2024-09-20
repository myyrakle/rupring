fn to_pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
            }
        })
        .collect()
}

#[test]
fn to_pascal_case_test() {
    assert_eq!(to_pascal_case(""), "");
    assert_eq!(to_pascal_case("_"), "");
    assert_eq!(to_pascal_case("__"), "");
    assert_eq!(to_pascal_case("a__b"), "AB");
    assert_eq!(to_pascal_case("ac__ba"), "AcBa");
    assert_eq!(to_pascal_case("myy_rakle"), "MyyRakle");
}

pub(crate) fn make_route_name(function_name: &str) -> String {
    let mut route_name = String::new();

    route_name.push_str(&format!("Route{}", to_pascal_case(function_name)));

    return route_name;
}

pub(crate) fn make_handler_name(function_name: &str) -> String {
    let mut handler_name = String::new();

    handler_name.push_str(&format!("Handler{}", to_pascal_case(function_name)));

    return handler_name;
}
