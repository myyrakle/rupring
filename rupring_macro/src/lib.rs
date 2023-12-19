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
            let route_name = rule::make_route_name(route);
            format!("Box::new({route_name}{{}})")
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

// #[proc_macro_attribute]
// #[allow(non_snake_case)]
// pub fn Injectable(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // ...
//     return item;
// }

#[allow(non_snake_case)]
fn ManipulateRouteFunctionParameters(item: TokenStream) -> TokenStream {
    let mut new_item = vec![];

    let mut iter = item.into_iter();

    let mut fn_passed = false;
    let mut out_of_parameter = false;

    while let Some(mut token_tree) = iter.next() {
        match token_tree.clone() {
            proc_macro::TokenTree::Ident(ident) => match ident.to_string().as_str() {
                "fn" => {
                    if !out_of_parameter {
                        fn_passed = true;
                    }
                }
                _ => {}
            },
            proc_macro::TokenTree::Group(delimiter) => {
                if fn_passed && !out_of_parameter {
                    let mut iter = delimiter.stream().into_iter();

                    let mut new_group = vec![];
                    let mut comma_count = 0;

                    while let Some(token_tree) = iter.next() {
                        match token_tree.clone() {
                            proc_macro::TokenTree::Punct(punct) => {
                                if punct.to_string().as_str() == "," {
                                    comma_count += 1;
                                }

                                new_group.push(token_tree);
                            }
                            _ => {
                                new_group.push(token_tree);
                            }
                        }
                    }

                    if comma_count == 0 {
                        let new_code =
                            TokenStream::from_str(", response: rupring::Response").unwrap();

                        for token_tree in new_code.into_iter() {
                            new_group.push(token_tree);
                        }
                    }

                    if comma_count == 1 && new_group.last().unwrap().to_string().as_str() == "," {
                        let new_code =
                            TokenStream::from_str(" response: rupring::Response").unwrap();

                        for token_tree in new_code.into_iter() {
                            new_group.push(token_tree);
                        }
                    }

                    token_tree = proc_macro::TokenTree::Group(proc_macro::Group::new(
                        delimiter.delimiter(),
                        new_group.into_iter().collect(),
                    ));

                    out_of_parameter = true;
                }
            }

            _ => {}
        }

        new_item.push(token_tree);
    }

    new_item.into_iter().collect()
}

#[allow(non_snake_case)]
fn MapRoute(method: String, attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = ManipulateRouteFunctionParameters(item);

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
