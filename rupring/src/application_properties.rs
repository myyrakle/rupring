use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationProperties {
    pub server: Server,
    pub environment: String,

    pub etc: HashMap<String, String>,
}

impl Default for ApplicationProperties {
    fn default() -> Self {
        ApplicationProperties {
            server: Server::default(),
            environment: "dev".to_string(),
            etc: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Compression {
    pub enabled: bool,
    pub mime_types: Vec<String>,
    pub min_response_size: usize,
    pub algorithm: String,
}

impl Default for Compression {
    fn default() -> Self {
        Compression {
            enabled: false,
            mime_types: vec![
                "text/html",
                "text/xml",
                "text/plain",
                "text/css",
                "text/javascript",
                "application/javascript",
                "application/json",
                "application/xml",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            min_response_size: 2048, // 2KB
            algorithm: "gzip".to_string(),
        }
    }
}

// Reference: https://docs.spring.io/spring-boot/appendix/application-properties/index.html#appendix.application-properties.server
#[derive(Debug, PartialEq, Clone)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub compression: Compression,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            address: "0.0.0.0".to_string(),
            port: 3000,
            compression: Compression::default(),
        }
    }
}

impl ApplicationProperties {
    pub fn from_properties(text: String) -> ApplicationProperties {
        let mut server = Server::default();
        let mut environment = "dev".to_string();
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

            // TODO: 매크로 기반 파싱 구현
            match key.as_str() {
                "server.port" => {
                    if let Ok(value) = value.parse::<u16>() {
                        server.port = value;
                    }
                }
                "server.address" => {
                    server.address = value.to_string();
                }
                "server.compression.enabled" => {
                    if let Ok(value) = value.parse::<bool>() {
                        server.compression.enabled = value;
                    }
                }
                "server.compression.mime-types" => {
                    server.compression.mime_types =
                        value.split(",").map(|s| s.to_string()).collect();
                }
                "server.compression.min-response-size" => {
                    if let Ok(value) = value.parse::<usize>() {
                        server.compression.min_response_size = value;
                    }
                }
                "server.compression.algorithm" => {
                    server.compression.algorithm = value.to_string();
                }
                "environment" => {
                    environment = value.to_string();
                }
                _ => {
                    etc.insert(key, value);
                }
            }
        }

        ApplicationProperties {
            server,
            etc,
            environment,
        }
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
                        ..Default::default()
                    },
                    etc: HashMap::new(),
                    environment: "dev".to_string(),
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
                        ..Default::default()
                    },
                    environment: "dev".to_string(),
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
                        ..Default::default()
                    },
                    environment: "dev".to_string(),
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
                        ..Default::default()
                    },
                    environment: "dev".to_string(),
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
                        ..Default::default()
                    },
                    environment: "dev".to_string(),
                    etc: HashMap::new(),
                },
            },
            TestCase {
                name: "environment 바인딩".to_string(),
                input: r#"
                    server.port=80#@#@80
                    server.address= 127.0.0.1
                    environment=prod
                    "#
                .to_string(),
                expected: ApplicationProperties {
                    server: Server {
                        address: "127.0.0.1".to_string(),
                        port: 3000,
                        ..Default::default()
                    },
                    environment: "prod".to_string(),
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

// 알아서 모든 대상에 대해 application.properties를 읽어서 ApplicationProperties를 반환하는 함수
pub fn load_application_properties_from_all() -> ApplicationProperties {
    // 1. 현재 경로에 application.properties가 있는지 확인하고, 있다면 읽어서 반환합니다.
    if let Ok(text) = std::fs::read_to_string("application.properties") {
        return ApplicationProperties::from_properties(text);
    }

    ApplicationProperties::default()
}
