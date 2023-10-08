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
