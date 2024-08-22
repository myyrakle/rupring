use super::SwaggerDefinition;

pub trait ToSwaggerDefinitionNode {
    fn to_swagger_definition() -> SwaggerDefinitionNode;
}

pub struct SwaggerDefinitionLeaf {
    pub type_: String,
}

pub enum SwaggerDefinitionNode {
    Root(SwaggerDefinition),
    Leaf(SwaggerDefinitionLeaf),
}

impl ToSwaggerDefinitionNode for i8 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for i16 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for i32 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for i64 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for i128 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for u8 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for u16 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for u32 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for u64 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for u128 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for bool {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "boolean".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for f32 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for f64 {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "number".to_string(),
        })
    }
}

impl ToSwaggerDefinitionNode for String {
    fn to_swagger_definition() -> SwaggerDefinitionNode {
        SwaggerDefinitionNode::Leaf(SwaggerDefinitionLeaf {
            type_: "string".to_string(),
        })
    }
}
