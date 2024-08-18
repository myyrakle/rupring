use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree};
use syn::ItemStruct;

// Find the structure name immediately to the right of the struct keyword.
pub(crate) fn find_struct_name(struct_ast: &ItemStruct) -> String {
    struct_ast.ident.to_string()
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

#[derive(Debug, PartialEq, Clone)]
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
                            let attributes = parse_attribute(group.stream(), true);

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
pub(crate) fn parse_attribute(
    item: TokenStream,
    with_alias: bool,
) -> HashMap<String, AttributeValue> {
    let mut tokens = item.into_iter();
    let mut attribute_map = HashMap::new();

    let mut attribute_name = None;
    while let Some(token) = tokens.next() {
        let token_string = token.to_string();

        if token_string == "=" {
            let mut attribute_name = attribute_name.clone().unwrap();
            let mut attribute_value = tokens
                .next()
                .expect("key/value pair does not match")
                .to_string();

            if with_alias {
                if attribute_name == "path" || attribute_name == "Path" {
                    attribute_name = "PathVariable".into();
                }

                if attribute_name == "desc"
                    || attribute_name == "Desc"
                    || attribute_name == "description"
                {
                    attribute_name = "Description".into();
                }
            }

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

#[derive(Debug, Clone)]
pub(crate) struct AnnotatedParameter {
    pub(crate) attributes: HashMap<String, AttributeValue>,
    pub(crate) name: String,
    pub(crate) type_: String,
}

// 1. 타입 일관성을 위해 Request와 Response 매개변수가 존재하지 않는다면 강제로 추가합니다.
// 2. 어노테이션이 붙은 특수한 파라미터를 제거해서 반환합니다.
pub(crate) fn manipulate_route_function_parameters(
    item: TokenStream,
) -> (TokenStream, Vec<AnnotatedParameter>) {
    let mut new_item = vec![];
    let mut parameters = vec![];

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
            proc_macro::TokenTree::Group(group) => {
                if fn_passed && !out_of_parameter {
                    // 파라미터 영역을 분석합니다.
                    let mut iter = group.stream().into_iter().peekable();
                    let mut annotation_removed = vec![];

                    // annotation이 달린 특수한 파라미터들을 추출합니다.
                    // ,를 기준으로 파라미터를 분리해서 분석합니다.
                    while let Some(token_tree) = iter.next() {
                        match token_tree.clone() {
                            proc_macro::TokenTree::Punct(punct) => {
                                let punct = punct.to_string();

                                match punct.as_str() {
                                    "#" => {
                                        let group = iter.next().unwrap();

                                        let mut attributes =
                                            if let proc_macro::TokenTree::Group(group) = group {
                                                parse_attribute(group.stream(), true)
                                            } else {
                                                panic!(
                                                    "invalid annotation parameter (group expected)"
                                                );
                                            };

                                        while let Some(TokenTree::Punct(punct)) = iter.peek() {
                                            if punct.to_string() == "#" {
                                                iter.next().unwrap();

                                                let group = iter.next().unwrap();

                                                let new_attributes =
                                                    if let proc_macro::TokenTree::Group(group) =
                                                        group
                                                    {
                                                        parse_attribute(group.stream(), true)
                                                    } else {
                                                        panic!(
                                                            "invalid annotation parameter (group expected)"
                                                        );
                                                    };

                                                for (key, value) in new_attributes {
                                                    attributes.insert(key, value);
                                                }

                                                continue;
                                            }

                                            break;
                                        }

                                        let expect_name = iter.next().unwrap();
                                        let name = if let proc_macro::TokenTree::Ident(ident) =
                                            expect_name
                                        {
                                            ident.to_string()
                                        } else {
                                            panic!("invalid annotation parameter (ident expected)");
                                        };

                                        let expect_colon = iter.next().unwrap();
                                        if expect_colon.to_string().as_str() != ":" {
                                            panic!("invalid annotation parameter (: expected)");
                                        }

                                        let mut type_ = "".to_string();
                                        while let Some(token_tree) = iter.next() {
                                            let token_tree = token_tree.clone();
                                            let token_tree = token_tree.to_string();

                                            if token_tree.as_str() == "," {
                                                break;
                                            }

                                            type_.push_str(token_tree.as_str());
                                        }

                                        parameters.push(AnnotatedParameter {
                                            attributes,
                                            name,
                                            type_,
                                        });
                                    }
                                    _ => {
                                        annotation_removed.push(token_tree);
                                    }
                                }
                            }
                            _ => annotation_removed.push(token_tree),
                        }
                    }

                    // 여기서부턴 request, response 파라미터에 대한 기본 처리를 수행합니다.
                    // ,를 기준으로 파라미터를 분리해서 분석합니다.
                    let mut request_name = "request".to_string();
                    let mut response_name = "response".to_string();

                    let replaced_parameter: TokenStream = annotation_removed.into_iter().collect();
                    let parameter_text = replaced_parameter.to_string();

                    for parameter in parameter_text.split(",") {
                        let parameter = parameter.trim();

                        if parameter.contains("rupring::Request") || parameter.contains("Request") {
                            request_name = parameter
                                .split(":")
                                .next()
                                .unwrap()
                                .to_string()
                                .trim()
                                .to_string();
                        }

                        if parameter.contains("rupring::Response") || parameter.contains("Response")
                        {
                            response_name = parameter
                                .split(":")
                                .next()
                                .unwrap()
                                .to_string()
                                .trim()
                                .to_string();
                        }
                    }

                    let new_parameter_code = format!(
                        "{request_name}: rupring::Request, {response_name}: rupring::Response",
                    );

                    token_tree = proc_macro::TokenTree::Group(proc_macro::Group::new(
                        group.delimiter(),
                        new_parameter_code.parse().unwrap(),
                    ));

                    out_of_parameter = true;
                }
            }

            _ => {}
        }

        new_item.push(token_tree);
    }

    (new_item.into_iter().collect(), parameters)
}

pub(crate) fn prepend_code_to_function_body(item: TokenStream, code: TokenStream) -> TokenStream {
    let mut new_item = vec![];
    let mut iter = item.into_iter();
    let mut prepended = false;

    while let Some(token_tree) = iter.next() {
        if !prepended {
            if let proc_macro::TokenTree::Group(group) = token_tree.clone() {
                if group.delimiter() == proc_macro::Delimiter::Brace {
                    let mut new_tokens = vec![];

                    for token_tree in code.clone().into_iter() {
                        new_tokens.push(token_tree);
                    }

                    let group_stream = group.stream();
                    let mut group_iter = group_stream.into_iter();
                    while let Some(group_token) = group_iter.next() {
                        new_tokens.push(group_token);
                    }

                    let replaced_group = proc_macro::Group::new(
                        proc_macro::Delimiter::Brace,
                        new_tokens.into_iter().collect(),
                    );

                    new_item.push(proc_macro::TokenTree::Group(replaced_group));

                    prepended = true;
                    continue;
                }
            }
        }

        new_item.push(token_tree);
    }

    new_item.into_iter().collect()
}
