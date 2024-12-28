use std::collections::HashMap;

use proc_macro::TokenStream;

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
                    if let Some(proc_macro::TokenTree::Group(group)) = iter.next() {
                        let attributes = parse_attribute(group.stream(), true);

                        for (key, value) in attributes {
                            map.insert(key, value);
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

    let mut attribute_name_outer = None;
    while let Some(token) = tokens.next() {
        let token_string = token.to_string();

        if token_string == "=" {
            let mut attribute_name = attribute_name_outer.clone().unwrap();
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

            attribute_name_outer = None;
        } else {
            attribute_name_outer = Some(token_string);
        }
    }

    if let Some(attribute_name) = attribute_name_outer {
        attribute_map.insert(attribute_name, AttributeValue::String("".into()));
    }

    return attribute_map;
}

#[derive(Debug, Clone)]
pub(crate) struct AnnotatedParameter {
    pub(crate) attributes: HashMap<String, AttributeValue>,
    pub(crate) name: String,
    pub(crate) type_: String,
}
