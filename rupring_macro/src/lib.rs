mod attribute;
mod parse;
mod rule;
use std::str::FromStr;

use attribute::AttributeValue;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Expr;

const SHARP: &str = "#";

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Module(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let _item = item.clone();
    let ast = syn::parse_macro_input!(_item as syn::ItemStruct);

    let struct_name = parse::find_struct_name(&ast);

    let attribute_map: std::collections::HashMap<String, attribute::AttributeValue> =
        attribute::parse_attribute(attr.clone(), false);

    let controllers = match attribute_map.get("controllers") {
        Some(controllers) => match controllers {
            attribute::AttributeValue::ListOfString(controllers) => controllers.to_owned(),
            attribute::AttributeValue::String(controller) => vec![controller.to_owned()],
        },
        None => vec![],
    };

    let modules = match attribute_map.get("modules") {
        Some(modules) => match modules {
            attribute::AttributeValue::ListOfString(modules) => modules.to_owned(),
            AttributeValue::String(module) => vec![module.to_owned()],
        },
        None => vec![],
    };

    let providers = match attribute_map.get("providers") {
        Some(providers) => match providers {
            attribute::AttributeValue::ListOfString(providers) => providers.to_owned(),
            AttributeValue::String(provider) => vec![provider.to_owned()],
        },
        None => vec![],
    };

    let middlewares = match attribute_map.get("middlewares") {
        Some(middlewares) => match middlewares {
            AttributeValue::ListOfString(middlewares) => middlewares.to_owned(),
            AttributeValue::String(middleware) => vec![middleware.to_owned()],
        },
        None => vec![],
    };

    let controllers = controllers
        .iter()
        .map(|controller| format!("Box::new({})", controller.to_owned()))
        .collect::<Vec<String>>()
        .join(", ");

    let modules = modules
        .iter()
        .map(|module| format!("Box::new({})", module.to_owned()))
        .collect::<Vec<String>>()
        .join(", ");

    let providers = providers
        .iter()
        .map(|provider| format!("Box::new({})", provider.to_owned()))
        .collect::<Vec<String>>()
        .join(", ");

    let middlewares = middlewares
        .iter()
        .map(|middleware| format!("Box::new({})", middleware.to_owned()))
        .collect::<Vec<String>>()
        .join(", ");

    let new_code = format!(
        r#"impl rupring::IModule for {struct_name} {{
    fn child_modules(&self) -> Vec<Box<dyn rupring::IModule>> {{
        vec![{modules}]
    }}

    fn controllers(&self) -> Vec<Box<dyn rupring::IController>> {{
        vec![{controllers}]
    }}

    fn providers(&self) -> Vec<Box<dyn rupring::IProvider>> {{
        vec![{providers}]
    }}

    fn middlewares(&self) -> Vec<rupring::MiddlewareFunction> {{
        vec![{middlewares}]
    }}
}}
"#
    );

    item.extend(TokenStream::from_str(new_code.as_str()).unwrap());

    item
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let _item = item.clone();
    let ast = syn::parse_macro_input!(_item as syn::ItemStruct);

    let struct_name = parse::find_struct_name(&ast);

    let attribute_map = attribute::parse_attribute(attr.clone(), false);

    let prefix = match attribute_map.get("prefix") {
        Some(AttributeValue::String(prefix)) => prefix.to_owned(),
        Some(_) => "".to_string(),
        None => "".to_string(),
    };

    let routes = match attribute_map.get("routes") {
        Some(routes) => match routes {
            AttributeValue::ListOfString(routes) => routes.to_owned(),
            AttributeValue::String(route) => vec![route.to_owned()],
        },
        None => vec![],
    };

    let routes = routes
        .iter()
        .map(|route| {
            let mut scopes = route.split("::").map(|e| e.trim()).collect::<Vec<&str>>();

            let route = scopes.pop().unwrap();

            let route_name = rule::make_route_name(route);

            let scopes = scopes.join("::");

            if scopes.is_empty() {
                format!("Box::new({route_name}{{}})")
            } else {
                format!("Box::new({scopes}::{route_name}{{}})")
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    let middlewares = match attribute_map.get("middlewares") {
        Some(middlewares) => match middlewares {
            AttributeValue::ListOfString(middlewares) => middlewares.to_owned(),
            AttributeValue::String(middleware) => vec![middleware.to_owned()],
        },
        None => vec![],
    };

    let middlewares = middlewares
        .iter()
        .map(|middleware| format!("Box::new({})", middleware.to_owned()))
        .collect::<Vec<String>>()
        .join(", ");

    let new_code = format!(
        r#"impl rupring::IController for {struct_name} {{
            fn prefix(&self) -> String {{
                "{prefix}".to_string()
            }}
        
            fn routes(&self) -> Vec<Box<dyn rupring::IRoute + Send + 'static>> {{
                vec![{routes}]
            }}

            fn middlewares(&self) -> Vec<rupring::MiddlewareFunction> {{
                vec![{middlewares}]
            }}
        }}"#
    );

    item.extend(TokenStream::from_str(new_code.as_str()).unwrap());

    item
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Service(attr: TokenStream, item: TokenStream) -> TokenStream {
    Injectable(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Repository(attr: TokenStream, item: TokenStream) -> TokenStream {
    Injectable(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Component(attr: TokenStream, item: TokenStream) -> TokenStream {
    Injectable(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Bean(attr: TokenStream, item: TokenStream) -> TokenStream {
    Injectable(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Injectable(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let _item = item.clone();
    let function_ast = syn::parse_macro_input!(_item as syn::ItemFn);

    let _provider_type = parse::find_function_return_type(&function_ast);
    let parameters_types = parse::find_function_parameter_types(&function_ast);
    let function_name = parse::find_function_name(&function_ast);

    let mut dependencies = vec![];
    let mut arguments = vec![];
    for parameter_type in parameters_types {
        dependencies.push(format!("std::any::TypeId::of::<{parameter_type}>()",));

        if parameter_type.contains("&") {
            arguments.push(format!("di_context.get::<{parameter_type}>().unwrap()",));
        } else {
            arguments.push(format!(
                "di_context.get::<{parameter_type}>().unwrap().to_owned()"
            ));
        }
    }

    let struct_name = if attr.is_empty() {
        function_name.clone()
    } else if attr.clone().into_iter().count() == 1 {
        attr.into_iter().next().unwrap().to_string()
    } else {
        let attribute_map = attribute::parse_attribute(attr.clone(), false);

        match attribute_map.get("name") {
            Some(AttributeValue::String(name)) => name.to_owned(),
            Some(_) => function_name.clone(),
            None => function_name.clone(),
        }
    };

    let function_call = format!("{function_name}({})", arguments.join(", "));
    let dependencies = dependencies.join(", ");

    let new_code = format!(
        r#"
#[allow(non_camel_case_types)]
pub struct {struct_name}{{}}
impl rupring::IProvider for {struct_name} {{
    fn dependencies(&self) -> Vec<std::any::TypeId> {{
        vec![{dependencies}]
    }}

    fn provide(&self, di_context: &rupring::DIContext) -> Box<dyn std::any::Any> {{
        Box::new({function_call})
    }}
}}"#
    );

    item.extend(TokenStream::from_str(new_code.as_str()).unwrap());

    item
}

fn convert_rust_type_to_js_type(rust_type: &str) -> String {
    match rust_type {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" => {
            "integer".to_string()
        }
        "Option<i8>" | "Option<i16>" | "Option<i32>" | "Option<i64>" | "Option<i128>"
        | "Option<isize>" | "Option<u8>" | "Option<u16>" | "Option<u32>" | "Option<u64>"
        | "Option<u128>" => "integer".to_string(),
        "f32" | "f64" => "number".to_string(),
        "Option<f32>" | "Option<f64>" => "number".to_string(),
        "bool" => "boolean".to_string(),
        "Option<bool>" => "boolean".to_string(),
        _ => "string".to_string(),
    }
}

#[allow(non_snake_case)]
fn MapRoute(method: String, attr: TokenStream, item: TokenStream) -> TokenStream {
    let _item = item.clone();
    let function_ast = syn::parse_macro_input!(_item as syn::ItemFn);

    let (item, additional_attributes) = attribute::extract_additional_attributes(item);
    let summary = additional_attributes
        .get("summary")
        .map(|e| e.as_string())
        .unwrap_or_default()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_owned();

    let description = additional_attributes
        .get("Description")
        .map(|e| e.as_string())
        .unwrap_or_default()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_owned();

    let tags = additional_attributes
        .get("tags")
        .map(|e| match e {
            AttributeValue::ListOfString(tags) => tags.to_owned(),
            AttributeValue::String(tag) => vec![tag.to_owned()],
        })
        .map(|e| {
            e.iter()
                .map(|e| e.trim_start_matches("\"").trim_end_matches("\"").to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let tags = format!(
        "vec![{}]",
        tags.iter()
            .map(|e| format!("\"{}\".to_string()", e))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let request_body = additional_attributes
        .get("parameters")
        .or(additional_attributes.get("params"))
        .map(|e| e.as_string())
        .unwrap_or_default()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_owned();

    let response = additional_attributes
        .get("response")
        .map(|e| e.as_string())
        .unwrap_or_default()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_owned();

    let auth = additional_attributes.get("auth").map(|e| {
        let mut auth_value = e
            .as_string()
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_owned();

        if auth_value.is_empty() {
            auth_value = "BearerAuth".to_string();
        }

        auth_value
    });

    let (item, annotated_parameters) = parse::manipulate_route_function_parameters(item);

    let mut swagger_code = "".to_string();
    let mut variables_code = "".to_string();

    // 함수 최상단에 코드를 주입합니다.

    for anotated_parameter in annotated_parameters {
        if anotated_parameter.attributes.contains_key("PathVariable") {
            let parameter_name = anotated_parameter.name;
            let parameter_type = anotated_parameter.type_;
            let path_name = anotated_parameter.attributes["PathVariable"].as_string();
            let path_name = path_name.trim_start_matches("\"").trim_end_matches("\"");
            let required = !parameter_type.starts_with("Option<");
            let type_ = convert_rust_type_to_js_type(parameter_type.as_str());

            let description = anotated_parameter
                .attributes
                .get("Description")
                .map(|e| {
                    e.as_string()
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                })
                .unwrap_or_default();

            let variable_expression = format!(
                r###"
                use rupring::ParamStringDeserializer;
                let ___{parameter_name} = rupring::ParamString(request.path_parameters["{path_name}"].clone());
                let {parameter_name}: {parameter_type} = match ___{parameter_name}.deserialize() {{
                    Ok(v) => v,
                    Err(_) => return rupring::Response::new().status(400).text("Invalid parameter: {parameter_name}"),
                }};
                "###
            );

            variables_code.push_str(&variable_expression);

            swagger_code.push_str(
                format!(
                    r##"
                swagger.parameters.push(
                    rupring::swagger::SwaggerParameter {{
                        name: "{parameter_name}".to_string(),
                        in_: rupring::swagger::SwaggerParameterCategory::Path,
                        description: "{description}".to_string(),
                        required: {required},
                        schema: Some(rupring::swagger::SwaggerTypeOrReference::Type(
                            rupring::swagger::SwaggerType {{
                                type_: "{type_}".to_string(),
                            }} 
                        )),
                        type_: None,
                    }}
                );
            "##
                )
                .as_str(),
            );

            continue;
        }
    }

    let mut item = parse::prepend_code_to_function_body(
        item,
        TokenStream::from_str(variables_code.as_str()).unwrap(),
    );

    let function_name = parse::find_function_name(&function_ast);
    let attribute_map = attribute::parse_attribute(attr.clone(), false);

    let path = match attribute_map.get("path") {
        Some(AttributeValue::String(path)) => path.to_owned(),
        Some(_) => "".to_string(),
        None => "".to_string(),
    };

    let route_name = rule::make_route_name(function_name.as_str());
    let handler_name = rule::make_handler_name(function_name.as_str());

    let mut swagger_request_body_code = "".to_string();
    if !request_body.is_empty() {
        swagger_request_body_code = format!(
            r#"
            fn swagger_request_info(&self) -> Option<rupring::swagger::macros::SwaggerRequestBody> {{
                rupring::swagger::macros::generate_swagger_request_info::<{request_body}>()
            }}
            "#
        );
    }

    let mut swagger_response_body_code = "".to_string();
    if !response.is_empty() {
        swagger_response_body_code = format!(
            r#"
            fn swagger_response_info(&self) -> Option<rupring::swagger::macros::SwaggerRequestBody> {{
                rupring::swagger::macros::generate_swagger_request_info::<{response}>()
            }}
            "#
        );
    }

    let mut swagger_security_code = "".to_string();
    if let Some(auth) = auth {
        swagger_security_code = format!(
            r#"
            fn swagger_security_info(&self) -> Vec<rupring::swagger::json::SwaggerSecurity> {{
                vec![
                    std::collections::HashMap::from_iter(vec![("{auth}".to_string(), vec![])])
                ]
            }}
            "#
        );
    }

    swagger_code.push_str(format!("swagger.summary = \"{summary}\".to_string();").as_str());
    swagger_code.push_str(format!("swagger.description = \"{description}\".to_string();").as_str());
    swagger_code.push_str(format!("swagger.tags = {tags};", tags = tags).as_str());

    let new_code = format!(
        r#"
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) struct {route_name} {{}}

impl rupring::IRoute for {route_name} {{
    fn method(&self) -> rupring::Method {{
        rupring::Method::{method}
    }}

    fn path(&self) -> String {{
        "{path}".to_string()
    }}

    fn handler(&self) -> Box<dyn rupring::IHandler + Send + 'static> {{
        Box::new({handler_name}{{}})
    }}

    fn swagger(&self) -> rupring::swagger::SwaggerOperation {{
        let mut swagger = rupring::swagger::SwaggerOperation::default();
        {swagger_code}
        swagger
    }}

    {swagger_request_body_code}

    {swagger_response_body_code}

    {swagger_security_code}
}}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) struct {handler_name}{{}}

impl rupring::IHandler for {handler_name} {{
    fn handle(&self, request: rupring::Request, response: rupring::Response) -> rupring::Response {{
        {function_name}(request, response)
    }}
}}
"#,
    );

    item.extend(TokenStream::from_str(new_code.as_str()).unwrap());

    item
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Get(attr: TokenStream, item: TokenStream) -> TokenStream {
    MapRoute("GET".to_string(), attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GetMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    Get(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Post(attr: TokenStream, item: TokenStream) -> TokenStream {
    MapRoute("POST".to_string(), attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn PostMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    Post(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Put(attr: TokenStream, item: TokenStream) -> TokenStream {
    MapRoute("PUT".to_string(), attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn PutMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    Put(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    MapRoute("DELETE".to_string(), attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DeleteMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    Delete(attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    MapRoute("PATCH".to_string(), attr, item)
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn PatchMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    Patch(attr, item)
}

/**
## What is RupringDto?
- This is a macro used to automatically generate Swagger documents.
- It also provides functions such as Request Validation.

## Attribute
1. example: Example of the field value.
2. description: Description of the field.
3. required: Whether the field is required.
4. name: Name of the field.
5. path_param: Path parameter.
6. param: Parameter.
7. query: Query parameter.
8. body: Body parameter.
9. ignore: Ignore the field.
 */
#[proc_macro_derive(
    RupringDto,
    attributes(
        example,
        description,
        desc,
        required,
        name,
        path_param,
        param,
        query,
        body,
        ignore,
    )
)]
pub fn derive_rupring_dto(item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = parse::find_struct_name(&ast);

    let mut code = "".to_string();

    code +=
        format!(r#"impl rupring::swagger::macros::ToSwaggerDefinitionNode for {struct_name} {{"#)
            .as_str();

    code += "fn get_definition_name() -> String {";
    code += r#"let current_module_name = module_path!().to_string();"#;
    code += format!(r#"let definition_name = format!("{{current_module_name}}::{struct_name}");"#)
        .as_str();
    code += "definition_name";
    code += "}";

    code += "fn to_swagger_definition(context: &mut rupring::swagger::macros::SwaggerDefinitionContext) -> rupring::swagger::macros::SwaggerDefinitionNode {";
    code += r#"let mut swagger_definition = rupring::swagger::json::SwaggerDefinition {"#;
    code += r#"type_: "object".to_string(),"#;
    code += r#"properties: std::collections::HashMap::new(),"#;
    code += r#"required: vec![],"#;
    code += r#"path_parameters: vec![],"#;
    code += r#"query_parameters: vec![],"#;
    code += "};";

    let mut define_struct_for_json = "".to_string();
    define_struct_for_json += "#[allow(non_camel_case_types)]";
    define_struct_for_json += r#"#[derive(serde::Serialize, serde::Deserialize)]"#;
    define_struct_for_json += format!(r#"pub struct {struct_name}__JSON {{"#).as_str();

    let mut json_field_names = vec![];
    let mut path_field_names = vec![];
    let mut query_field_names = vec![];

    for field in ast.fields.iter() {
        let mut description = "".to_string();
        let mut example = r#""""#.to_string();

        let field_name = field.ident.as_ref().unwrap().to_string();
        let mut field_type = field.ty.to_token_stream().to_string().replace(" ", "");

        let struct_field_name = field_name.clone();
        let mut param_name = field_name.clone();

        let attributes = field.attrs.clone();

        let mut is_required = true;
        let mut is_ignore = false;

        let mut is_path_parameter = false;
        let mut is_query_parameter = false;

        if field_type.starts_with("Option<") {
            is_required = false;
        }

        for attribute in attributes {
            let metadata = attribute.meta;

            let path = metadata.path().to_token_stream().to_string();

            let meta_value = metadata.require_name_value().ok().map(|e| e.value.clone());

            match path.to_lowercase().as_str() {
                "example" => {
                    if let Some(Expr::Lit(lit)) = &meta_value {
                        example = format!("{:?}", lit.to_token_stream().to_string());
                    }
                }
                "required" => {
                    if let Some(Expr::Lit(lit)) = &meta_value {
                        is_required = lit.to_token_stream().to_string().parse().unwrap();
                    } else {
                        is_required = true;
                    }
                }
                "description" | "desc" => {
                    if let Some(Expr::Lit(lit)) = &meta_value {
                        let mut text = lit.to_token_stream().to_string();

                        if text.starts_with("\"") {
                            text = text
                                .trim_start_matches("\"")
                                .trim_end_matches("\"")
                                .to_string();
                        }

                        description = text;
                    }
                }
                "name" => {
                    if let Some(Expr::Lit(lit)) = &meta_value {
                        let mut text = lit.to_token_stream().to_string();

                        if text.starts_with("\"") {
                            text = text
                                .trim_start_matches("\"")
                                .trim_end_matches("\"")
                                .to_string();
                        }

                        param_name = text;
                    }
                }
                "path_param" | "param" => {
                    if let Some(Expr::Lit(lit)) = &meta_value {
                        let mut text = lit.to_token_stream().to_string();

                        if text.starts_with("\"") {
                            text = text
                                .trim_start_matches("\"")
                                .trim_end_matches("\"")
                                .to_string();
                        }

                        param_name = text;
                    }

                    is_path_parameter = true;
                }
                "query" => {
                    is_query_parameter = true;

                    if let Some(Expr::Lit(lit)) = &meta_value {
                        let mut text = lit.to_token_stream().to_string();

                        if text.starts_with("\"") {
                            text = text
                                .trim_start_matches("\"")
                                .trim_end_matches("\"")
                                .to_string();
                        }

                        param_name = text;
                    }
                }
                "ignore" => {
                    is_ignore = true;
                }
                _ => {}
            }
        }

        if is_ignore {
            continue;
        }

        if is_required {
            code += format!(r#"swagger_definition.required.push("{field_name}".to_string());"#)
                .as_str();
        }

        // T<A> 형태를 T::<A> 형태로 변환
        if field_type.contains("<") {
            field_type = field_type.replace("<", "::<")
        }

        if is_path_parameter {
            path_field_names.push((
                struct_field_name.clone(),
                param_name.clone(),
                field_type.clone(),
            ));

            code += format!(
                r#"swagger_definition.path_parameters.push(rupring::swagger::json::SwaggerParameter {{
                name: "{field_name}".to_string(),
                in_: rupring::swagger::json::SwaggerParameterCategory::Path,
                description: "{description}".to_string(),
                required: {is_required},
                schema: Some(rupring::swagger::json::SwaggerTypeOrReference::Type(
                    rupring::swagger::json::SwaggerType {{
                        type_: "{field_type}".to_string(),
                    }}
                )),
                type_: None,
            }});"#
            ).as_str();

            continue;
        }

        if is_query_parameter {
            query_field_names.push((
                struct_field_name.clone(),
                param_name.clone(),
                field_type.clone(),
            ));

            code += format!(
                r#"swagger_definition.query_parameters.push(rupring::swagger::json::SwaggerParameter {{
                name: "{field_name}".to_string(),
                in_: rupring::swagger::json::SwaggerParameterCategory::Query,
                description: "{description}".to_string(),
                required: {is_required},
                schema: Some(rupring::swagger::json::SwaggerTypeOrReference::Type(
                    rupring::swagger::json::SwaggerType {{
                        type_: "{field_type}".to_string(),
                    }}
                )),
                type_: None,
            }});"#
            ).as_str();

            continue;
        }

        json_field_names.push((
            struct_field_name.clone(),
            param_name.clone(),
            field_type.clone(),
        ));

        define_struct_for_json += format!(
            r#"
            pub {struct_field_name}: {field_type}, 
        "#
        )
        .as_str();

        // Body 파라미터 생성 구현
        code += format!(r#"let property_of_type = {field_type}::to_swagger_definition(context);"#)
            .as_str();

        code += format!(
            r#"let property_value = match property_of_type {{
            rupring::swagger::macros::SwaggerDefinitionNode::Single(leaf) => {{
                rupring::swagger::json::SwaggerProperty::Single(rupring::swagger::json::SwaggerSingleProperty {{
                    type_: leaf.type_,
                    description: "{description}".to_string(),
                    example: Some({example}.into()),
                }})
            }},
            rupring::swagger::macros::SwaggerDefinitionNode::Array(array) => {{
                rupring::swagger::json::SwaggerProperty::Array(rupring::swagger::json::SwaggerArrayProperty {{
                    type_: array.type_,
                    items: array.items,
                    description: "{description}".to_string(),
                }})
            }},
            rupring::swagger::macros::SwaggerDefinitionNode::Object(object) => {{
                let definition_name = {field_type}::get_definition_name();

                context.definitions.insert(definition_name.clone(), object);

                rupring::swagger::json::SwaggerProperty::Reference(rupring::swagger::json::SwaggerReferenceProperty {{
                    reference: "{SHARP}/definitions/".to_string() + definition_name.as_str(),
                    description: "{description}".to_string(),
                }})
            }},
        }};"#
        )
        .as_str();

        code += format!(
            r#"swagger_definition.properties.insert("{field_name}".to_string(), property_value);"#
        )
        .as_str();
    }

    define_struct_for_json += "}";

    code += "rupring::swagger::macros::SwaggerDefinitionNode::Object(swagger_definition)";

    code += "}";

    code += "}";

    code += define_struct_for_json.as_str();

    let mut request_bind_code = "".to_string();
    request_bind_code +=
        format!(r#"impl rupring::request::BindFromRequest for {struct_name} {{"#).as_str();

    request_bind_code +=
        "fn bind(request: rupring::request::Request) -> rupring::anyhow::Result<Self> {";
    request_bind_code += "use rupring::request::ParamStringDeserializer;";
    request_bind_code += "use rupring::request::QueryStringDeserializer;";

    request_bind_code += format!("let mut json_bound = rupring::serde_json::from_str::<{struct_name}__JSON>(request.body.as_str())?;").as_str();

    request_bind_code += format!("let bound = {struct_name} {{").as_str();

    for (struct_field_name, _, _) in json_field_names {
        request_bind_code +=
            format!("{struct_field_name}: json_bound.{struct_field_name},").as_str();
    }

    for (struct_field_name, param_name, field_type) in path_field_names {
        request_bind_code += format!(
            r#"{struct_field_name}: {{
                let param = rupring::request::ParamString(
                    request.path_parameters["{param_name}"].clone()
                );

                let deserialized: {field_type} = match param.deserialize() {{
                    Ok(v) => v,
                    Err(_) => return Err(rupring::anyhow::anyhow!("invalid parameter: {param_name}")),
                }};

                deserialized
            }}
            "#
        )
        .as_str();
    }

    for (struct_field_name, param_name, field_type) in query_field_names {
        let mut code_if_field_type_is_none =
            format!(r#"return Err(rupring::anyhow::anyhow!("invalid parameter: {param_name}"));"#);

        if field_type.starts_with("Option<") || field_type.starts_with("Option::<") {
            code_if_field_type_is_none = r#"
                rupring::request::QueryString(vec![])
            "#
            .to_string();
        }

        request_bind_code += format!(
            r#"{struct_field_name}: {{
                let query = if let Some(query) = request.query_parameters.get("{param_name}") {{
                    rupring::request::QueryString(query.to_owned())
                }} else {{
                    {code_if_field_type_is_none}
                }};

                let deserialized: {field_type} = match query.deserialize_query_string() {{
                    Ok(v) => v,
                    Err(_) => return Err(rupring::anyhow::anyhow!("invalid parameter: {param_name}")),
                }};

                deserialized
            }},
            "#
        )
        .as_str();
    }

    request_bind_code += "};";

    request_bind_code += "Ok(bound)";
    request_bind_code += "}";

    request_bind_code += "}";

    code += request_bind_code.as_str();

    TokenStream::from_str(code.as_str()).unwrap()
}
