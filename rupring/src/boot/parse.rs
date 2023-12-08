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
}
