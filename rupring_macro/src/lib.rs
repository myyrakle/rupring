mod generate;
mod parse;
mod rule;
use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Module(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_name = parse::find_struct_name(item.clone());
    let attribute_map = parse::parse_attribute(attr.clone());

    let controllers = match attribute_map.get("controllers") {
        Some(controllers) => match controllers {
            parse::AttributeValue::ListOfString(controllers) => controllers.to_owned(),
            parse::AttributeValue::String(controller) => vec![controller.to_owned()],
        },
        None => vec![],
    };

    let modules = match attribute_map.get("modules") {
        Some(modules) => match modules {
            parse::AttributeValue::ListOfString(modules) => modules.to_owned(),
            parse::AttributeValue::String(module) => vec![module.to_owned()],
        },
        None => vec![],
    };

    let providers = match attribute_map.get("providers") {
        Some(providers) => match providers {
            parse::AttributeValue::ListOfString(providers) => providers.to_owned(),
            parse::AttributeValue::String(provider) => vec![provider.to_owned()],
        },
        None => vec![],
    };

    let middlewares = match attribute_map.get("middlewares") {
        Some(middlewares) => match middlewares {
            parse::AttributeValue::ListOfString(middlewares) => middlewares.to_owned(),
            parse::AttributeValue::String(middleware) => vec![middleware.to_owned()],
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

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_name = parse::find_struct_name(item.clone());
    let attribute_map = parse::parse_attribute(attr.clone());

    let prefix = match attribute_map.get("prefix") {
        Some(prefix) => match prefix {
            parse::AttributeValue::String(prefix) => prefix.to_owned(),
            _ => "".to_string(),
        },
        None => "".to_string(),
    };

    let routes = match attribute_map.get("routes") {
        Some(routes) => match routes {
            parse::AttributeValue::ListOfString(routes) => routes.to_owned(),
            parse::AttributeValue::String(route) => vec![route.to_owned()],
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

            if scopes.len() > 0 {
                format!("Box::new({scopes}::{route_name}{{}})")
            } else {
                format!("Box::new({route_name}{{}})")
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    let middlewares = match attribute_map.get("middlewares") {
        Some(middlewares) => match middlewares {
            parse::AttributeValue::ListOfString(middlewares) => middlewares.to_owned(),
            parse::AttributeValue::String(middleware) => vec![middleware.to_owned()],
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

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Service(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Injectable(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Repository(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Injectable(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Component(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Injectable(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Bean(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Injectable(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Injectable(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let _provider_type = parse::find_function_return_type(item.clone());
    let parameters_types = parse::find_function_parameter_types(item.clone());

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

    let function_name = parse::find_function_name(item.clone());

    let struct_name = if attr.is_empty() {
        function_name.clone()
    } else if attr.clone().into_iter().count() == 1 {
        attr.into_iter().next().unwrap().to_string()
    } else {
        let attribute_map = parse::parse_attribute(attr.clone());

        match attribute_map.get("name") {
            Some(name) => match name {
                parse::AttributeValue::String(name) => name.to_owned(),
                _ => function_name.clone(),
            },
            None => function_name.clone(),
        }
    };

    let function_call = format!("{function_name}({})", arguments.join(", "));
    let dependencies = dependencies.join(", ");

    let new_code = format!(
        r#"
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

    return item;
}

#[allow(non_snake_case)]
fn MapRoute(method: String, attr: TokenStream, item: TokenStream) -> TokenStream {
    let (item, additional_attributes) = parse::extract_additional_attributes(item);
    let summary = additional_attributes
        .get("summary")
        .map(|e| e.as_string())
        .unwrap_or_default()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_owned();

    let description = additional_attributes
        .get("description")
        .map(|e| e.as_string())
        .unwrap_or_default()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_owned();

    let tags = additional_attributes
        .get("tags")
        .map(|e| match e {
            parse::AttributeValue::ListOfString(tags) => tags.to_owned(),
            parse::AttributeValue::String(tag) => vec![tag.to_owned()],
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

    let (item, annotated_parameters) = parse::manipulate_route_function_parameters(item);

    let mut swagger_code = "".to_string();
    let mut variables_code = "".to_string();

    // 함수 최상단에 코드를 주입합니다.

    for anotated_parameter in annotated_parameters {
        if anotated_parameter.attributes.contains_key("path") {
            let parameter_name = anotated_parameter.name;
            let parameter_type = anotated_parameter.type_;
            let path_name = anotated_parameter.attributes["path"].as_string();
            let path_name = path_name.trim_start_matches("\"").trim_end_matches("\"");

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

            continue;
        }
    }

    let mut item = parse::prepend_code_to_function_body(
        item,
        TokenStream::from_str(variables_code.as_str()).unwrap(),
    );

    let function_name = parse::find_function_name(item.clone());
    let attribute_map = parse::parse_attribute(attr.clone());

    let path = match attribute_map.get("path") {
        Some(path) => match path {
            parse::AttributeValue::String(path) => path.to_owned(),
            _ => "".to_string(),
        },
        None => "".to_string(),
    };

    let route_name = rule::make_route_name(function_name.as_str());
    let handler_name = rule::make_handler_name(function_name.as_str());

    swagger_code.push_str(format!("swagger.summary = \"{summary}\".to_string();").as_str());
    swagger_code.push_str(format!("swagger.description = \"{description}\".to_string();").as_str());
    swagger_code.push_str(format!("swagger.tags = {tags};", tags = tags).as_str());

    let new_code = format!(
        r#"
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
}}

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

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Get(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("GET".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn GetMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Get(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Post(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("POST".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn PostMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Post(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Put(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("PUT".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn PutMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Put(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("DELETE".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DeleteMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Delete(attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("PATCH".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn PatchMapping(attr: TokenStream, item: TokenStream) -> TokenStream {
    return Patch(attr, item);
}
