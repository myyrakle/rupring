use std::collections::HashMap;

pub(crate) fn parse_query_parameter(raw_querystring: &str) -> HashMap<String, Vec<String>> {
    let mut query_parameters = HashMap::<String, Vec<String>>::new();

    for query_parameter in raw_querystring.split("&") {
        let query_parameter = query_parameter.split("=").collect::<Vec<&str>>();

        if query_parameter.len() != 2 {
            continue;
        }

        let key = query_parameter[0].to_string();
        let value = query_parameter[1].to_string();

        if query_parameters.contains_key(&key) {
            query_parameters.get_mut(&key).unwrap().push(value);
        } else {
            query_parameters.insert(key, vec![value]);
        }
    }

    return query_parameters;
}

pub(crate) fn parse_path_parameter(
    route_path: String,
    request_path: String,
) -> HashMap<String, String> {
    let mut path_parameters = HashMap::<String, String>::new();

    let route_path = route_path.split("/").collect::<Vec<&str>>();
    let request_path = request_path.split("/").collect::<Vec<&str>>();

    for (route_path_part, request_path_part) in route_path.iter().zip(request_path.iter()) {
        if route_path_part.starts_with(":") {
            let key = route_path_part[1..].to_string();
            let value = request_path_part.to_string();

            path_parameters.insert(key, value);
        }
    }

    return path_parameters;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_parameter() {
        struct TestCase {
            name: String,
            raw_querystring: &'static str,
            expected: HashMap<String, Vec<String>>,
        }

        let test_cases = vec![
            TestCase {
                name: "query string is empty".to_string(),
                raw_querystring: "",
                expected: HashMap::new(),
            },
            TestCase {
                name: "query string is not empty".to_string(),
                raw_querystring: "a=1&b=2&c=3&d=4&a=5",
                expected: {
                    let mut query_parameters = HashMap::new();

                    query_parameters
                        .insert("a".to_string(), vec!["1".to_string(), "5".to_string()]);
                    query_parameters.insert("b".to_string(), vec!["2".to_string()]);
                    query_parameters.insert("c".to_string(), vec!["3".to_string()]);
                    query_parameters.insert("d".to_string(), vec!["4".to_string()]);

                    query_parameters
                },
            },
        ];

        for test_case in test_cases.iter() {
            let result = parse_query_parameter(test_case.raw_querystring);

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, raw_querystring: {}",
                test_case.name, test_case.raw_querystring
            );
        }
    }

    #[test]
    fn test_parse_path_parameter() {
        struct TestCase {
            name: String,
            route_path: String,
            request_path: String,
            expected: HashMap<String, String>,
        }

        let test_cases = vec![
            TestCase {
                name: "route_path is empty".to_string(),
                route_path: "".to_string(),
                request_path: "".to_string(),
                expected: HashMap::new(),
            },
            TestCase {
                name: "route_path and request_path are the same".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello".to_string(),
                expected: HashMap::new(),
            },
            TestCase {
                name: "route_path and request_path are different".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/world".to_string(),
                expected: HashMap::new(),
            },
            TestCase {
                name: "route_path and request_path have different length".to_string(),
                route_path: "/hello".to_string(),
                request_path: "/hello/world".to_string(),
                expected: HashMap::new(),
            },
            TestCase {
                name: "single path parameter".to_string(),
                route_path: "/hello/:name".to_string(),
                request_path: "/hello/world".to_string(),
                expected: {
                    let mut path_parameters = HashMap::new();

                    path_parameters.insert("name".to_string(), "world".to_string());

                    path_parameters
                },
            },
            TestCase {
                name: "multiple path parameters".to_string(),
                route_path: "/hello/:name/:age".to_string(),
                request_path: "/hello/world/42".to_string(),
                expected: {
                    let mut path_parameters = HashMap::new();

                    path_parameters.insert("name".to_string(), "world".to_string());
                    path_parameters.insert("age".to_string(), "42".to_string());

                    path_parameters
                },
            },
        ];

        for test_case in test_cases.iter() {
            let result =
                parse_path_parameter(test_case.route_path.clone(), test_case.request_path.clone());

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, route_path: {}, request_path: {}",
                test_case.name, test_case.route_path, test_case.request_path
            );
        }
    }
}
