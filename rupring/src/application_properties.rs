use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationProperties {
    pub server: Server,

    pub etc: HashMap<String, String>,
}

// Reference: https://docs.spring.io/spring-boot/appendix/application-properties/index.html#appendix.application-properties.server
#[derive(Debug, PartialEq, Clone)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            address: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

impl ApplicationProperties {
    pub fn from_properties(text: String) -> ApplicationProperties {
        let mut server = Server::default();
        let mut etc = HashMap::new();

        for line in text.lines() {
            let mut parts = line.split("=");

            let key = match parts.next() {
                Some(key) => key.trim().to_owned(),
                None => continue,
            };
            let value = match parts.next() {
                Some(value) => value.trim().to_owned(),
                None => continue,
            };

            // value에 앞뒤로 ""가 있다면 제거
            let value = if value.starts_with('"') && value.ends_with('"') {
                value[1..value.len() - 1].to_string()
            } else {
                value.to_string()
            };

            match key.as_str() {
                "server.port" => {
                    if let Ok(value) = value.parse::<u16>() {
                        server.port = value;
                    }
                }
                "server.address" => {
                    server.address = value.to_string();
                }
                _ => {
                    etc.insert(key, value);
                }
            }
        }

        ApplicationProperties { server, etc }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_properties() {
        struct TestCase {
            name: String,
            input: String,
            expected: ApplicationProperties,
        }

        let test_cases = vec![
            TestCase {
                name: "일반적인 기본 속성 바인딩".to_string(),
                input: r#"
                    server.port=8080
                    server.address=127.0.0.1
                    "#
                .to_string(),
                expected: ApplicationProperties {
                    server: Server {
                        address: "127.0.0.1".to_string(),
                        port: 8080,
                    },
                    etc: HashMap::new(),
                },
            },
            TestCase {
                name: "추가 속성 바인딩".to_string(),
                input: r#"
                    server.port=8080
                    server.address=127.0.0.1
                    foo.bar=hello
                    "#
                .to_string(),
                expected: ApplicationProperties {
                    server: Server {
                        address: "127.0.0.1".to_string(),
                        port: 8080,
                    },
                    etc: HashMap::from([("foo.bar".to_string(), "hello".to_string())]),
                },
            },
            TestCase {
                name: "따옴표로 감싸기".to_string(),
                input: r#"
                    server.port=8080
                    server.address="127.0.0.1"
                    "#
                .to_string(),
                expected: ApplicationProperties {
                    server: Server {
                        address: "127.0.0.1".to_string(),
                        port: 8080,
                    },
                    etc: HashMap::new(),
                },
            },
            TestCase {
                name: "중간에 띄어쓰기".to_string(),
                input: r#"
                    server.port=8080
                    server.address= 127.0.0.1
                    "#
                .to_string(),
                expected: ApplicationProperties {
                    server: Server {
                        address: "127.0.0.1".to_string(),
                        port: 8080,
                    },
                    etc: HashMap::new(),
                },
            },
            TestCase {
                name: "포트 파싱 실패".to_string(),
                input: r#"
                    server.port=80#@#@80
                    server.address= 127.0.0.1
                    "#
                .to_string(),
                expected: ApplicationProperties {
                    server: Server {
                        address: "127.0.0.1".to_string(),
                        port: 3000,
                    },
                    etc: HashMap::new(),
                },
            },
        ];

        for tc in test_cases {
            let got = ApplicationProperties::from_properties(tc.input.clone());
            assert_eq!(
                got, tc.expected,
                "{} - input: {:?}, actual: {:?}",
                tc.name, tc.input, got
            );
        }
    }
}
