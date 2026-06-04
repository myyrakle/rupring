use std::collections::HashMap;

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn decode_url_component(value: &str, plus_as_space: bool) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                if let (Some(high), Some(low)) = (hex_value(bytes[i + 1]), hex_value(bytes[i + 2]))
                {
                    decoded.push((high << 4) | low);
                    i += 3;
                    continue;
                }

                decoded.push(bytes[i]);
            }
            b'+' if plus_as_space => decoded.push(b' '),
            byte => decoded.push(byte),
        }

        i += 1;
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

pub(crate) fn parse_query_parameter(raw_querystring: &str) -> HashMap<String, Vec<String>> {
    let mut query_parameters = HashMap::<String, Vec<String>>::new();

    for query_parameter in raw_querystring.split("&") {
        let Some((key, value)) = query_parameter.split_once("=") else {
            continue;
        };

        let key = decode_url_component(key, true);
        let value = decode_url_component(value, true);

        if query_parameters.contains_key(&key) {
            query_parameters.get_mut(&key).unwrap().push(value);
        } else {
            query_parameters.insert(key, vec![value]);
        }
    }

    query_parameters
}

pub(crate) fn parse_path_parameter(
    route_path: String,
    request_path: &str,
) -> HashMap<String, String> {
    let mut path_parameters = HashMap::<String, String>::new();

    let route_path = route_path.split("/").collect::<Vec<&str>>();
    let request_path = request_path.split("/").collect::<Vec<&str>>();

    for (route_path_part, request_path_part) in route_path.iter().zip(request_path.iter()) {
        if let Some(key) = route_path_part.strip_prefix(":") {
            let key = key.to_string();
            let value = decode_url_component(request_path_part, false);

            path_parameters.insert(key, value);
        }
    }

    path_parameters
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

        let test_cases = [
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
            TestCase {
                name: "query string decodes url encoded keys and values".to_string(),
                raw_querystring: "%ED%95%9C%EA%B8%80=%EA%B0%92+one&token=a=b=c",
                expected: {
                    let mut query_parameters = HashMap::new();

                    query_parameters.insert("한글".to_string(), vec!["값 one".to_string()]);
                    query_parameters.insert("token".to_string(), vec!["a=b=c".to_string()]);

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

        let test_cases = [
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
            TestCase {
                name: "path parameters decode url encoded values".to_string(),
                route_path: "/hello/:name/:file".to_string(),
                request_path: "/hello/%ED%95%9C%EA%B8%80/a%2Fb+c".to_string(),
                expected: {
                    let mut path_parameters = HashMap::new();

                    path_parameters.insert("name".to_string(), "한글".to_string());
                    path_parameters.insert("file".to_string(), "a/b+c".to_string());

                    path_parameters
                },
            },
        ];

        for test_case in test_cases.iter() {
            let result = parse_path_parameter(
                test_case.route_path.clone(),
                test_case.request_path.as_str(),
            );

            assert_eq!(
                result, test_case.expected,
                "TC name: {}, route_path: {}, request_path: {}",
                test_case.name, test_case.route_path, test_case.request_path
            );
        }
    }
}
