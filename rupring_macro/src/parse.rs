use std::collections::HashMap;

use proc_macro::TokenStream;

// Find the structure name immediately to the right of the struct keyword.
pub(crate) fn find_struct_name(item: TokenStream) -> String {
    let mut tokens = item.into_iter();
    let mut struct_name = String::new();

    while let Some(token) = tokens.next() {
        if token.to_string() == "struct" {
            struct_name = tokens.next().unwrap().to_string();
            break;
        }
    }

    return struct_name;
}

#[derive(Debug, PartialEq)]
pub enum AttributeValue {
    ListOfString(Vec<String>),
    String(String),
}

// controllers = [HomeController {}], modules = [] => HashMap<String, AttributeValue>
pub(crate) fn parse_attribute(item: TokenStream) -> HashMap<String, AttributeValue> {
    let mut tokens = item.into_iter();
    let mut attribute_map = HashMap::new();

    let mut attribute_name = None;
    while let Some(token) = tokens.next() {
        let token_string = token.to_string();

        if token_string == "=" {
            let attribute_name = attribute_name.clone().unwrap();
            let attribute_value = tokens.next().unwrap().to_string();

            let attribute_value = if attribute_value.starts_with("[") {
                let attribute_value = attribute_value
                    .trim_start_matches("[")
                    .trim_end_matches("]")
                    .to_string();

                let attribute_value = attribute_value
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();

                // filter empty string
                let attribute_value = attribute_value
                    .into_iter()
                    .filter(|s| s.len() > 0)
                    .collect::<Vec<String>>();

                AttributeValue::ListOfString(attribute_value)
            } else {
                AttributeValue::String(attribute_value)
            };

            attribute_map.insert(attribute_name, attribute_value);
        } else {
            attribute_name = Some(token_string);
        }
    }

    return attribute_map;
}
