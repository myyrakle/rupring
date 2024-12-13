use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerSchema {
    #[serde(rename = "swagger")]
    pub version: String,

    #[serde(rename = "info")]
    pub info: SwaggerInfo,

    #[serde(rename = "host")]
    pub host: Option<String>,

    #[serde(rename = "basePath")]
    pub base_path: String,

    #[serde(rename = "schemes")]
    pub schemes: Vec<String>,

    #[serde(rename = "tags")]
    pub tags: Vec<SwaggerTag>,

    #[serde(rename = "paths")]
    pub paths: SwaggerPaths,

    #[serde(rename = "definitions")]
    pub definitions: SwaggerDefinitions,

    #[serde(rename = "securityDefinitions")]
    pub security_definitions: SwaggerSecurityDefinitions,

    #[serde(rename = "externalDocs")]
    pub external_docs: Option<SwaggerExternalDoc>,
}

impl Default for SwaggerSchema {
    fn default() -> Self {
        SwaggerSchema {
            version: "2.0".to_string(),
            info: Default::default(),
            host: None,
            base_path: r#""#.to_string(),
            schemes: vec![],
            tags: Default::default(),
            paths: Default::default(),
            definitions: Default::default(),
            security_definitions: SwaggerSecurityDefinition::new_default_definitions(),
            external_docs: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerLicense {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "url")]
    pub url: String,
}

impl Default for SwaggerLicense {
    fn default() -> Self {
        SwaggerLicense {
            name: "Apache 2.0".to_string(),
            url: "http://www.apache.org/licenses/LICENSE-2.0.html".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerInfo {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "license")]
    pub license: SwaggerLicense,

    #[serde(rename = "termsOfService")]
    pub terms_of_service: String,

    #[serde(rename = "contact")]
    pub contact: SwaggerContact,
}

impl Default for SwaggerInfo {
    fn default() -> Self {
        SwaggerInfo {
            title: "".to_string(),
            version: "".to_string(),
            description: "".to_string(),
            license: Default::default(),
            terms_of_service: "http://swagger.io/terms/".to_string(),
            contact: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerContact {
    #[serde(rename = "email")]
    pub email: String,
}

impl Default for SwaggerContact {
    fn default() -> Self {
        SwaggerContact {
            email: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerTag {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "externalDocs")]
    pub external_docs: Option<SwaggerExternalDoc>,
}

pub struct SwaggerTags(pub(crate) Vec<SwaggerTag>);

impl SwaggerTags {
    pub const fn new() -> Self {
        SwaggerTags(vec![])
    }

    // if exists, do nothing
    // if not exists, add tag
    pub fn add_tag(&mut self, tag: String) {
        let mut exists = false;

        for swagger_tag in self.0.iter() {
            if swagger_tag.name == tag {
                exists = true;
                break;
            }
        }

        if !exists {
            self.0.push(SwaggerTag {
                name: tag,
                description: "".to_string(),
                external_docs: None,
            });
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerExternalDoc {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "url")]
    pub url: String,
}

impl Default for SwaggerExternalDoc {
    fn default() -> Self {
        SwaggerExternalDoc {
            description: "".to_string(),
            url: "".to_string(),
        }
    }
}

pub type SwaggerPaths = HashMap<String, SwaggerPath>;

pub type SwaggerPath = HashMap<String, SwaggerOperation>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerOperation {
    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "summary")]
    pub summary: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "operationId")]
    pub operation_id: String,

    #[serde(rename = "consumes")]
    pub consumes: Vec<String>,

    #[serde(rename = "produces")]
    pub produces: Vec<String>,

    #[serde(rename = "parameters")]
    pub parameters: Vec<SwaggerParameter>,

    #[serde(rename = "responses")]
    pub responses: SwaggerResponses,

    #[serde(rename = "security")]
    pub security: Vec<SwaggerSecurity>,
}

impl Default for SwaggerOperation {
    fn default() -> Self {
        SwaggerOperation {
            tags: Default::default(),
            summary: "".to_string(),
            description: "".to_string(),
            operation_id: "".to_string(),
            consumes: vec!["application/json".to_string()],
            produces: vec!["application/json".to_string()],
            parameters: Default::default(),
            responses: Default::default(),
            security: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerParameter {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "in")]
    pub in_: SwaggerParameterCategory,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "required")]
    pub required: bool,

    #[serde(rename = "type")]
    pub type_: Option<String>,

    #[serde(rename = "schema")]
    pub schema: Option<SwaggerTypeOrReference>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SwaggerParameterCategory {
    #[serde(rename = "path")]
    Path,

    #[serde(rename = "query")]
    Query,

    #[serde(rename = "body")]
    Body,

    #[serde(rename = "header")]
    Header,

    #[serde(rename = "formData")]
    FormData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerReference {
    #[serde(rename = "$ref")]
    pub reference: String,
}

pub type SwaggerResponses = HashMap<String, SwaggerResponse>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerResponse {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "schema")]
    pub schema: Option<SwaggerReference>,
}

pub type SwaggerSecurity = HashMap<String, Vec<String>>;

pub type SwaggerSecurityDefinitions = HashMap<String, SwaggerSecurityDefinition>;

impl SwaggerSecurityDefinition {
    pub fn new_bearer_auth() -> Self {
        SwaggerSecurityDefinition::APIKey(SwaggerAPIKey {
            type_: "apiKey".to_string(),
            name: "Authorization".to_string(),
            in_: "header".to_string(),
        })
    }

    pub fn new_default_definitions() -> SwaggerSecurityDefinitions {
        let mut security_definitions = SwaggerSecurityDefinitions::new();

        security_definitions.insert("BearerAuth".to_string(), Self::new_bearer_auth());

        security_definitions
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SwaggerSecurityDefinition {
    APIKey(SwaggerAPIKey),
    Oauth2(SwaggerOauth2),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerAPIKey {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "in")]
    pub in_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerOauth2 {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "flow")]
    pub flow: String,

    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,

    #[serde(rename = "scopes")]
    pub scopes: SwaggerOauth2Scopes,
}

pub type SwaggerOauth2Scopes = HashMap<String, String>;

pub type SwaggerDefinitions = HashMap<String, SwaggerDefinition>;

pub type SwaggerDefinition = SwaggerDefinitionObject;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerDefinitionObject {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "properties")]
    pub properties: SwaggerProperties,

    #[serde(rename = "required")]
    pub required: Vec<String>,

    #[serde(skip_serializing)]
    pub path_parameters: Vec<SwaggerParameter>,

    #[serde(skip_serializing)]
    pub query_parameters: Vec<SwaggerParameter>,
}

pub type SwaggerProperties = HashMap<String, SwaggerProperty>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SwaggerProperty {
    Array(SwaggerArrayProperty),
    Single(SwaggerSingleProperty),
    Reference(SwaggerReferenceProperty),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SwaggerArrayProperty {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "items")]
    pub items: SwaggerTypeOrReference,

    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SwaggerPrimitiveType {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl From<&str> for SwaggerPrimitiveType {
    fn from(value: &str) -> Self {
        let string = value.to_string();

        if string.to_lowercase() == "true" || string.to_lowercase() == "false" {
            return SwaggerPrimitiveType::Boolean(string.parse().unwrap());
        }

        let chars = string.chars().collect::<Vec<char>>();

        if chars.len() > 0 {
            if chars[0].is_numeric() {
                return SwaggerPrimitiveType::Number(string.parse().unwrap());
            }

            if chars[0] == '"' && chars[chars.len() - 1] == '"' {
                return SwaggerPrimitiveType::String(string[1..chars.len() - 1].to_string());
            }
        }

        SwaggerPrimitiveType::String(string)
    }
}

impl Default for SwaggerPrimitiveType {
    fn default() -> Self {
        SwaggerPrimitiveType::String("".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SwaggerSingleProperty {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "example")]
    pub example: Option<SwaggerPrimitiveType>, // TODO: Union으로 치환
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerReferenceProperty {
    #[serde(rename = "$ref")]
    pub reference: String,

    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerType {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SwaggerTypeOrReference {
    Type(SwaggerType),
    Reference(SwaggerReference),
}

impl Default for SwaggerTypeOrReference {
    fn default() -> Self {
        SwaggerTypeOrReference::Type(SwaggerType {
            type_: "".to_string(),
        })
    }
}
