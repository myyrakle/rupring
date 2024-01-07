use std::collections::HashMap;

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
    security_definitions: SwaggerSecurityDefinitions,

    #[serde(rename = "externalDocs")]
    external_docs: SwaggerExternalDoc,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerInfo {
    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "license")]
    license: SwaggerLicense,

    #[serde(rename = "termsOfService")]
    terms_of_service: String,

    #[serde(rename = "contact")]
    contact: SwaggerContact,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerContact {
    #[serde(rename = "email")]
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerTag {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "externalDocs")]
    external_docs: SwaggerExternalDoc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerExternalDoc {
    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "url")]
    url: String,
}

pub type SwaggerPaths = HashMap<String, SwaggerPath>;

pub type SwaggerPath = HashMap<String, SwaggerOperation>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerOperation {
    #[serde(rename = "tags")]
    tags: Vec<String>,

    #[serde(rename = "summary")]
    summary: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "operationId")]
    operation_id: String,

    #[serde(rename = "consumes")]
    consumes: Vec<String>,

    #[serde(rename = "produces")]
    produces: Vec<String>,

    #[serde(rename = "parameters")]
    parameters: Vec<SwaggerParameter>,

    #[serde(rename = "responses")]
    responses: SwaggerResponses,

    #[serde(rename = "security")]
    security: Vec<SwaggerSecurity>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerParameter {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "in")]
    in_: SwaggerParameterCategory,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "required")]
    required: bool,

    #[serde(rename = "type")]
    type_: Option<String>,

    #[serde(rename = "schema")]
    schema: Option<SwaggerReference>,
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
    reference: String,
}

pub type SwaggerResponses = HashMap<String, SwaggerResponse>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerResponse {
    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "schema")]
    schema: Option<SwaggerReference>,
}

pub type SwaggerSecurity = HashMap<String, Vec<String>>;

pub type SwaggerSecurityDefinitions = HashMap<String, SwaggerSecurityDefinition>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SwaggerSecurityDefinition {
    APIKey(SwaggerAPIKey),
    Oauth2(SwaggerOauth2),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerAPIKey {
    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "in")]
    in_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwaggerOauth2 {
    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "flow")]
    flow: String,

    #[serde(rename = "authorizationUrl")]
    authorization_url: String,

    #[serde(rename = "scopes")]
    scopes: SwaggerOauth2Scopes,
}

pub type SwaggerOauth2Scopes = HashMap<String, String>;
