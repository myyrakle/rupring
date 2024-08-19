use proc_macro::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{FnArg, ItemFn, ItemStruct};

use crate::attribute::{self, AnnotatedParameter};

// Find the structure name immediately to the right of the struct keyword.
pub(crate) fn find_struct_name(struct_ast: &ItemStruct) -> String {
    struct_ast.ident.to_string()
}

// Find the structure name immediately to the right of the fn keyword.
pub(crate) fn find_function_name(function_ast: &ItemFn) -> String {
    function_ast.sig.ident.to_string()
}

// Returns the types of function parameters as an array.
pub(crate) fn find_function_parameter_types(function_ast: &ItemFn) -> Vec<String> {
    let parameters = function_ast.sig.inputs.clone();

    let mut parameters_types = vec![];

    for arg in parameters {
        if let FnArg::Typed(pat_type) = arg {
            let _type = pat_type.ty.to_token_stream().to_string();

            // " :: " 패턴을 전부 "::"로 치환
            let _type = _type.replace(" :: ", "::");

            parameters_types.push(_type);
        }
    }

    parameters_types
}

// Find the return type of the function.
pub(crate) fn find_function_return_type(function_ast: &ItemFn) -> String {
    let return_type = function_ast.sig.output.to_token_stream().to_string();

    // 화살표 제거
    let return_type = return_type.replace(" -> ", "");

    // 공백 제거
    let return_type = return_type.trim().to_owned();

    return_type
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
                                                attribute::parse_attribute(group.stream(), true)
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
                                                        attribute::parse_attribute(
                                                            group.stream(),
                                                            true,
                                                        )
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
