mod parse;

use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

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

    let new_code = format!(
        r#"impl rupring::IModule for {struct_name} {{
    fn child_modules(&self) -> Vec<Box<dyn rupring::IModule>> {{
        vec![{modules}]
    }}

    fn controllers(&self) -> Vec<Box<dyn rupring::IController>> {{
        vec![{controllers}]
    }}
}}"#
    );

    item.extend(TokenStream::from_str(new_code.as_str()).unwrap());

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Injectable(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Get(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Post(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Put(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}
