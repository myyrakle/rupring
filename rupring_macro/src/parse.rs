use std::collections::HashMap;
use std::str::FromStr;

use proc_macro::{TokenStream, TokenTree};

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

// Find the structure name immediately to the right of the fn keyword.
pub(crate) fn find_function_name(item: TokenStream) -> String {
    let mut tokens = item.into_iter();
    let mut function_name = String::new();

    while let Some(token) = tokens.next() {
        if token.to_string() == "fn" {
            function_name = tokens.next().unwrap().to_string();
            break;
        }
    }

    return function_name;
}

// Returns the types of function parameters as an array.
pub(crate) fn find_function_parameter_types(item: TokenStream) -> Vec<String> {
    let mut tokens = item.into_iter();
    let mut parameter_types = Vec::new();

    while let Some(token) = tokens.next() {
        if let TokenTree::Group(group) = token {
            let mut group_tokens = group.stream().into_iter();

            while let Some(group_token) = group_tokens.next() {
                if group_token.to_string() == ":" {
                    let mut parameter_type = group_tokens.next().unwrap().to_string();

                    if parameter_type == "&" {
                        parameter_type += group_tokens.next().unwrap().to_string().as_str();
                    }

                    parameter_types.push(parameter_type.clone());
                }
            }

            break;
        }
    }

    return parameter_types;
}

// Find the return type of the function.
pub(crate) fn find_function_return_type(item: TokenStream) -> String {
    let mut tokens = item.into_iter();
    let mut return_type = String::new();

    while let Some(token) = tokens.next() {
        if token.to_string() == "-" && tokens.next().unwrap().to_string() == ">" {
            return_type = tokens.next().unwrap().to_string();
            break;
        }
    }

    return return_type;
}

#[derive(Debug, PartialEq)]
pub enum AttributeValue {
    ListOfString(Vec<String>),
    String(String),
}

impl AttributeValue {
    pub fn as_string(&self) -> String {
        match self {
            AttributeValue::String(value) => value.clone(),
            AttributeValue::ListOfString(value) => value.join(","),
        }
    }
}

pub(crate) fn extract_additional_attributes(
    item: TokenStream,
) -> (TokenStream, HashMap<String, AttributeValue>) {
    let mut map = HashMap::new();

    let mut code_without_attributes: Vec<proc_macro::TokenTree> = vec![];
    let mut iter = item.into_iter();
    let mut done = false;

    while let Some(tree) = iter.next() {
        if done {
            code_without_attributes.push(tree);
            continue;
        }

        match tree {
            proc_macro::TokenTree::Punct(ref punct) => {
                if punct.to_string().as_str() == "#" {
                    // [Key1 = value1, key2, value2, ...] 형태의 attribute를 파싱해서 map에 할당한다.
                    if let Some(group) = iter.next() {
                        if let proc_macro::TokenTree::Group(group) = group {
                            let attributes = parse_attribute(group.stream());

                            for (key, value) in attributes {
                                map.insert(key, value);
                            }
                        }
                    }
                } else {
                    code_without_attributes.push(tree);
                    done = true;
                }
            }
            _ => {
                code_without_attributes.push(tree);
                done = true;
            }
        }
    }

    (code_without_attributes.into_iter().collect(), map)
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
            let mut attribute_value = tokens
                .next()
                .expect("key/value pair does not match")
                .to_string();

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
                while let Some(token) = tokens.next() {
                    let token_string = token.to_string();

                    if token_string == "," || token_string == "=" || token_string == ")" {
                        break;
                    }

                    attribute_value.push_str(&token_string);
                }

                AttributeValue::String(attribute_value)
            };

            attribute_map.insert(attribute_name, attribute_value);
        } else {
            attribute_name = Some(token_string);
        }
    }

    return attribute_map;
}

// 타입 일관성을 위해 Request와 Response 매개변수가 존재하지 않는다면 강제로 추가합니다.
pub(crate) fn manipulate_route_function_parameters(item: TokenStream) -> TokenStream {
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
