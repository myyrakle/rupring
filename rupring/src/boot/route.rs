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
}
