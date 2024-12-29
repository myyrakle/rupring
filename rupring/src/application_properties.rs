/*!
## Intro
- application.properties is a configuration method influenced by spring.

## How to find it
- The rupring program searches the current execution path to see if there is a file called application.properties.
- If it does not exist, application.properties is searched based on the directory of the current executable file.
- If it is still not there, load it with default values ​​and start.

## Environment Variables
- Environment variables in the execution context are also loaded into application.properties.
- If application.properties and the environment variable have the same key, the environment variable is ignored.

## Format
- Similar to spring, it has a Key=Value format separated by newlines.

## Special Options
| Key | Description | Default |
| --- | --- | --- |
| environment | The environment to run in. | dev |
| server.port | The port to listen on. | 3000 |
| server.address | The address to listen on. | 0.0.0.0 |
| server.shutdown | The shutdown mode. (immediate,graceful) | immediate |
| server.timeout-per-shutdown-phase | The timeout per shutdown phase. (e.g. 30s, 1m, 1h) | 30s |
| server.compression.enabled | Whether to enable compression. | false |
| server.compression.mime-types | The mime types to compress. | text/html,text/xml,text/plain,text/css,text/javascript,application/javascript,application/json,application/xml |
| server.compression.min-response-size | The minimum response size to compress. (byte) | 2048 |
| server.compression.algorithm | The compression algorithm to use. (gzip,deflate) | gzip |
| server.thread.limit | The thread limit to use. | None(max) |
| server.request-timeout | The request timeout. (300 = 300 millisecond, 3s = 3 second, 2m = 2 minute) | No Timeout |
| server.http1.keep-alive | Whether to keep-alive for HTTP/1. (false=disable, true=enable) | false |
| server.ssl.key | The SSL key file. (SSL is enabled by feature="tls") | None |
| server.ssl.cert | The SSL cert file. (SSL is enabled by feature="tls") | None |
| banner.enabled | Whether to enable the banner. | true |
| banner.location | The location of the banner file. | None |
| banner.charset | The charset of the banner file. (UTF-8, UTF-16) | UTF-8 |
*/

use std::{collections::HashMap, time::Duration};

#[derive(Debug, PartialEq, Clone)]
pub struct ApplicationProperties {
    pub server: Server,
    pub environment: String,
    pub banner: Banner,

    pub etc: HashMap<String, String>,
}

impl Default for ApplicationProperties {
    fn default() -> Self {
        ApplicationProperties {
            server: Server::default(),
            environment: "dev".to_string(),
            etc: HashMap::new(),
            banner: Banner::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompressionAlgorithm {
    Gzip,
    Deflate,
    Unknown(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Compression {
    pub enabled: bool,
    pub mime_types: Vec<String>,
    pub min_response_size: usize,
    pub algorithm: CompressionAlgorithm,
}

impl ToString for CompressionAlgorithm {
    fn to_string(&self) -> String {
        match self {
            CompressionAlgorithm::Gzip => "gzip".to_string(),
            CompressionAlgorithm::Deflate => "deflate".to_string(),
            CompressionAlgorithm::Unknown(s) => s.to_string(),
        }
    }
}

impl From<String> for CompressionAlgorithm {
    fn from(s: String) -> Self {
        match s.as_str() {
            "gzip" => CompressionAlgorithm::Gzip,
            "deflate" => CompressionAlgorithm::Deflate,
            _ => CompressionAlgorithm::Unknown(s),
        }
    }
}

impl Default for Compression {
    fn default() -> Self {
        Compression {
            enabled: false,
            mime_types: [
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
            algorithm: CompressionAlgorithm::Gzip,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Banner {
    pub enabled: bool,
    pub charset: String,
    pub location: Option<String>,
}

impl Default for Banner {
    fn default() -> Self {
        Banner {
            enabled: true,
            charset: "UTF-8".to_string(),
            location: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShutdownType {
    Immediate,
    Graceful,
}

impl From<String> for ShutdownType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "immediate" => ShutdownType::Immediate,
            "graceful" => ShutdownType::Graceful,
            _ => ShutdownType::Immediate,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SSL {
    pub key: String,
    pub cert: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Http1 {
    pub keep_alive: bool,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Http2 {}

// Reference: https://docs.spring.io/spring-boot/appendix/application-properties/index.html#appendix.application-properties.server
#[derive(Debug, PartialEq, Clone)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub compression: Compression,
    pub shutdown: ShutdownType,
    pub timeout_per_shutdown_phase: String,
    pub thread_limit: Option<usize>,
    pub request_timeout: Option<Duration>,
    pub http1: Http1,
    pub http2: Http2,
    pub ssl: SSL,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            address: "0.0.0.0".to_string(),
            port: 3000,
            compression: Compression::default(),
            shutdown: ShutdownType::Immediate,
            timeout_per_shutdown_phase: "30s".to_string(),
            thread_limit: None,
            request_timeout: None,
            http1: Http1::default(),
            http2: Http2::default(),
            ssl: Default::default(),
        }
    }
}

impl Server {
    pub fn is_graceful_shutdown(&self) -> bool {
        self.shutdown == ShutdownType::Graceful
    }

    pub fn shutdown_timeout_duration(&self) -> std::time::Duration {
        let timeout = self
            .timeout_per_shutdown_phase
            .trim_end_matches(|c| !char::is_numeric(c));
        let timeout = timeout.parse::<u64>().unwrap_or(30);

        let duration = match self.timeout_per_shutdown_phase.chars().last() {
            Some('s') => std::time::Duration::from_secs(timeout),
            Some('m') => std::time::Duration::from_secs(timeout * 60),
            Some('h') => std::time::Duration::from_secs(timeout * 60 * 60),
            _ => std::time::Duration::from_secs(30),
        };

        duration
    }
}

impl ApplicationProperties {
    pub fn from_properties(text: String) -> ApplicationProperties {
        let mut server = Server::default();
        let mut environment = "dev".to_string();
        let mut etc = HashMap::new();
        let mut banner = Banner::default();

        let mut key_values = HashMap::new();

        // application.properties 파일에서 추출
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

            key_values.insert(key, value);
        }

        // 환경변수에서도 추출
        let env = std::env::vars().collect::<HashMap<_, _>>();
        for (key, value) in env {
            if key_values.contains_key(&key) {
                continue;
            }

            key_values.insert(key, value);
        }

        // 추출한 key-value를 바탕으로 기본 정의된 항목은 바인딩, 그 외는 etc에 저장
        for (key, value) in key_values {
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
                "server.shutdown" => server.shutdown = value.into(),
                "server.timeout-per-shutdown-phase" => {
                    server.timeout_per_shutdown_phase = value.to_string();
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
                    server.compression.algorithm = value.into();
                }
                "server.thread.limit" => {
                    if let Ok(value) = value.parse::<usize>() {
                        server.thread_limit = Some(value);
                    }
                }
                "server.request-timeout" => {
                    // * = millisecond
                    // *s = second
                    // *m = minute
                    let timeout = value.trim_end_matches(|c| !char::is_numeric(c));

                    if let Ok(timeout) = timeout.parse::<u64>() {
                        if timeout == 0 {
                            continue;
                        }

                        let duration = match value.chars().last() {
                            Some('s') => std::time::Duration::from_secs(timeout),
                            Some('m') => std::time::Duration::from_secs(timeout * 60),
                            _ => std::time::Duration::from_millis(timeout),
                        };

                        server.request_timeout = Some(duration);
                    }
                }
                "server.http1.keep-alive" => {
                    if let Ok(value) = value.parse::<bool>() {
                        server.http1.keep_alive = value;
                    }
                }
                "server.ssl.key" => {
                    server.ssl.key = value.to_string();
                }
                "server.ssl.cert" => {
                    server.ssl.cert = value.to_string();
                }
                "environment" => {
                    environment = value.to_string();
                }
                "banner.enabled" => {
                    banner.enabled = value.parse::<bool>().unwrap_or(true);
                }
                "banner.location" => {
                    banner.location = Some(value.to_string());
                }
                "banner.charset" => {
                    banner.charset = value.to_string();
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
            banner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_all_env() {
        for (key, _) in std::env::vars() {
            std::env::remove_var(key);
        }
    }

    #[test]
    fn test_from_properties() {
        struct TestCase {
            name: String,
            input: String,
            before: fn() -> (),
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
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
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
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
                },
            },
            TestCase {
                name: "추가 속성 바인딩 - 환경변수".to_string(),
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
                    environment: "dev".to_string(),
                    etc: HashMap::from([("asdf.fdsa".to_string(), "!!".to_string())]),
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
                    std::env::set_var("asdf.fdsa", "!!");
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
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
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
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
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
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
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
                    ..Default::default()
                },
                before: || {
                    remove_all_env();
                },
            },
        ];

        for tc in test_cases {
            (tc.before)();

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

    // 2. 실행파일 경로에 application.properties가 있는지 확인하고, 있다면 읽어서 반환합니다.
    let exe_path = std::env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");

    let exe_properties_path = exe_dir.join("application.properties");
    if let Ok(text) = std::fs::read_to_string(exe_properties_path) {
        return ApplicationProperties::from_properties(text);
    }

    println!("application.properties Not Found. Use default properties.");

    ApplicationProperties::default()
}
