use proc_macro::TokenStream;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Module(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Injectable(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    return item;
}
