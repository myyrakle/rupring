use std::collections::HashMap;

use super::{
    SwaggerArrayProperty, SwaggerDefinition, SwaggerDefinitionObject, SwaggerParameter,
    SwaggerReference, SwaggerSingleProperty, SwaggerType, SwaggerTypeOrReference,
};

pub struct SwaggerRequestInfo {
    pub request_body: Option<SwaggerRequestBody>,
    pub path_parameters: Vec<SwaggerParameter>,
    pub query_parameters: Vec<SwaggerParameter>,
}

pub struct SwaggerDefinitionContext {
    pub definitions: HashMap<String, SwaggerDefinitionObject>,
}

pub trait ToSwaggerDefinitionNode {
    fn to_swagger_definition(context: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode;
    fn get_definition_name() -> String {
        "".into()
    }
}

pub struct SwaggerDefinitionLeaf {
    pub type_: String,
}

pub enum SwaggerDefinitionNode {
    Object(SwaggerDefinition),
    Array(SwaggerArrayProperty),
    Single(SwaggerSingleProperty),
}

impl ToSwaggerDefinitionNode for i8 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for i16 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for i32 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for i64 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for i128 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for u8 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for u16 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for u32 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for u64 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for u128 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for bool {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "boolean".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for f32 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for f64 {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "number".to_string(),
            ..Default::default()
        })
    }
}

impl ToSwaggerDefinitionNode for String {
    fn to_swagger_definition(_: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Single(SwaggerSingleProperty {
            type_: "string".to_string(),
            ..Default::default()
        })
    }
}

impl<T: ToSwaggerDefinitionNode> ToSwaggerDefinitionNode for Vec<T> {
    fn to_swagger_definition(context: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        let item = T::to_swagger_definition(context);

        let item = match item {
            SwaggerDefinitionNode::Single(item) => {
                SwaggerTypeOrReference::Type(SwaggerType { type_: item.type_ })
            }
            SwaggerDefinitionNode::Array(item) => item.items,
            SwaggerDefinitionNode::Object(_) => {
                SwaggerTypeOrReference::Reference(SwaggerReference {
                    reference: "#/definitions/".to_string() + T::get_definition_name().as_str(),
                })
            }
        };

        SwaggerDefinitionNode::Array(SwaggerArrayProperty {
            type_: "array".to_string(),
            items: item,
            ..Default::default()
        })
    }
}

impl<T: ToSwaggerDefinitionNode> ToSwaggerDefinitionNode for Option<T> {
    fn to_swagger_definition(context: &mut SwaggerDefinitionContext) -> SwaggerDefinitionNode {
        T::to_swagger_definition(context)
    }
}
pub struct SwaggerRequestBody {
    pub definition_name: String,
    pub definition_value: SwaggerDefinitionObject,

    pub dependencies: Vec<SwaggerRequestBody>,

    pub path_parameters: Vec<SwaggerParameter>,
    pub query_parameters: Vec<SwaggerParameter>,
}

pub fn generate_swagger_request_info<T: ToSwaggerDefinitionNode>() -> Option<SwaggerRequestBody> {
    let mut context = SwaggerDefinitionContext {
        definitions: Default::default(),
    };

    let root_definition = T::to_swagger_definition(&mut context);
    let root_definition_name = T::get_definition_name();

    let mut swagger_request_body =
        if let crate::swagger::macros::SwaggerDefinitionNode::Object(def) = root_definition {
            SwaggerRequestBody {
                definition_name: root_definition_name,
                definition_value: def.clone(),
                dependencies: vec![],
                path_parameters: def.path_parameters.clone(),
                query_parameters: def.query_parameters.clone(),
            }
        } else {
            return None;
        };

    for (name, definition) in context.definitions {
        swagger_request_body.dependencies.push(SwaggerRequestBody {
            definition_name: name,
            definition_value: definition,
            dependencies: vec![],
            path_parameters: vec![],
            query_parameters: vec![],
        });
    }

    Some(swagger_request_body)
}
