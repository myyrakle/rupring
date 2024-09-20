fn to_pascal_case(name: &str) -> String {
    let words: Vec<&str> = name.split('_').collect();
    let mut result = String::new();

    for (_i, word) in words.iter().enumerate() {
        match word.len() {
            0 => {
                result.push_str(&word.to_lowercase());
            }
            1 => {
                result.push_str(&word.to_uppercase());
            }
            _ => {
                result.push_str(&word[0..1].to_uppercase());
                result.push_str(&word[1..].to_lowercase());
            }
        }
    }

    result
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
