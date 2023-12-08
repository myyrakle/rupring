use hyper::Method;

pub(crate) fn is_route_matching_request(route_path: String, request_path: String) -> bool {
    // remove query parameters
    let request_path = request_path.split("?").collect::<Vec<&str>>()[0];

    let route_path = route_path.split("/").collect::<Vec<&str>>();
    let request_path = request_path.split("/").collect::<Vec<&str>>();

    if route_path.len() != request_path.len() {
        return false;
    }

    for (route_path_part, request_path_part) in route_path.iter().zip(request_path.iter()) {
        if route_path_part.starts_with(":") {
            continue;
        }

        if route_path_part != request_path_part {
            return false;
        }
    }

    return true;
}

pub(crate) fn normalize_path(prefix: String, path: String) -> String {
    let mut normalized_path = "/".to_string();

    if prefix.starts_with("/") {
        normalized_path.push_str(&prefix[1..]);
    } else {
        normalized_path.push_str(&prefix);
    }

    if !normalized_path.ends_with("/") {
        normalized_path.push_str("/");
    }

    if path.starts_with("/") {
        normalized_path.push_str(&path[1..]);
    } else {
        normalized_path.push_str(&path);
    }

    if normalized_path.ends_with("/") && normalized_path.len() > 1 {
        normalized_path.pop();
    }

    return normalized_path;
}

pub(crate) fn find_route(
    root_module: Box<dyn crate::IModule>,
    request_path: String,
    request_method: Method,
) -> Option<Box<dyn crate::IRoute>> {
    for controller in root_module.controllers() {
        let prefix = controller.prefix();

        for route in controller.routes() {
            if route.method() != request_method {
                continue;
            }

            let route_path = format!("{}{}", prefix, route.path());

            if !is_route_matching_request(route_path, request_path.clone()) {
                continue;
            }

            return Some(route);
        }
    }

    for child_module in root_module.child_modules() {
        let result = find_route(child_module, request_path.clone(), request_method.clone());

        if result.is_some() {
            return result;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_route_matching_request() {
        struct TestCase {
            name: String,
            route_path: String,
            request_path: String,
            expected: bool,
        }

        let test_cases = vec![
            TestCase {
                name: "route_path is empty".to_string(),
                route_path: "".to_string(),
                request_path: "".to_string(),
                expected: true,
            },
            TestCase {
                name: "route_path and request_path are the same".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello".to_string(),
                expected: true,
            },
            TestCase {
                name: "route_path and request_path are different".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/world".to_string(),
                expected: false,
            },
            TestCase {
                name: "route_path and request_path have different length".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello/world".to_string(),
                expected: false,
            },
            TestCase {
                name: "single path parameter".to_string(),
                route_path: "/hello/:name".to_string(),
                request_path: "/hello/world".to_string(),
                expected: true,
            },
            TestCase {
                name: "single query parameter".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello?name=world".to_string(),
                expected: true,
            },
        ];

        for test_case in test_cases.iter() {
            let result = is_route_matching_request(
                test_case.route_path.clone(),
                test_case.request_path.clone(),
            );

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, route_path: {}, request_path: {}",
                test_case.name, test_case.route_path, test_case.request_path
            );
        }
    }

    #[test]
    fn test_normalize_path() {
        struct TestCase {
            name: String,
            prefix: String,
            path: String,
            expected: String,
        }

        let test_cases = vec![
            TestCase {
                name: "prefix and path are empty".to_string(),
                prefix: "".to_string(),
                path: "".to_string(),
                expected: "/".to_string(),
            },
            TestCase {
                name: "prefix is empty".to_string(),
                prefix: "".to_string(),
                path: "/hello".to_string(),
                expected: "/hello".to_string(),
            },
            TestCase {
                name: "path is empty".to_string(),
                prefix: "/hello".to_string(),
                path: "".to_string(),
                expected: "/hello".to_string(),
            },
            TestCase {
                name: "prefix and path are not empty".to_string(),
                prefix: "/hello".to_string(),
                path: "/world".to_string(),
                expected: "/hello/world".to_string(),
            },
            TestCase {
                name: "prefix and path are not empty and have trailing slashes".to_string(),
                prefix: "/hello/".to_string(),
                path: "/world/".to_string(),
                expected: "/hello/world".to_string(),
            },
        ];

        for test_case in test_cases.iter() {
            let result = normalize_path(test_case.prefix.clone(), test_case.path.clone());

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, prefix: {}, path: {}",
                test_case.name, test_case.prefix, test_case.path
            );
        }
    }
}
