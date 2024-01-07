use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerSchema {
    #[serde(rename = "swagger")]
    version: String,

    #[serde(rename = "info")]
    info: SwaggerInfo,

    #[serde(rename = "host")]
    host: String,

    #[serde(rename = "basePath")]
    base_path: String,

    #[serde(rename = "schemes")]
    schemes: Vec<String>,

    #[serde(rename = "tags")]
    tags: Vec<SwaggerTag>,

    #[serde(rename = "paths")]
    paths: Vec<SwaggerPath>,

    #[serde(rename = "definitions")]
    definitions: Vec<SwaggerDefinition>,

    #[serde(rename = "securityDefinitions")]
    security_definitions: Vec<SwaggerSecurityDefinition>,

    #[serde(rename = "externalDocs")]
    external_docs: Vec<SwaggerExternalDoc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerLicense {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "url")]
    url: String,
}

impl Default for SwaggerLicense {
    fn default() -> Self {
        SwaggerLicense {
            name: "Apache 2.0".to_string(),
            url: "http://www.apache.org/licenses/LICENSE-2.0.html".to_string(),
        }
    }
}
