use std::collections::HashMap;

pub struct ApplicationProperties {
    pub server: Server,

    pub etc: HashMap<String, String>,
}

// Reference: https://docs.spring.io/spring-boot/appendix/application-properties/index.html#appendix.application-properties.server
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
                    server.port = value.parse().unwrap();
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
