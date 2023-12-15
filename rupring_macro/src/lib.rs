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
            let route_name = rule::make_route_name(route);
            format!("Box::new({route_name}{{}})")
        })
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
        }}"#
    );

    item.extend(TokenStream::from_str(new_code.as_str()).unwrap());

    return item;
}

// #[proc_macro_attribute]
// #[allow(non_snake_case)]
// pub fn Injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // ...
//     return item;
// }

#[allow(non_snake_case)]
fn MapRoute(method: String, attr: TokenStream, mut item: TokenStream) -> TokenStream {
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
pub fn Post(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("POST".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Put(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("PUT".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("DELETE".to_string(), attr, item);
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    return MapRoute("PATCH".to_string(), attr, item);
}
