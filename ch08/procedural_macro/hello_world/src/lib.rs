use proc_macro::TokenStream;

#[proc_macro]
pub fn say_hello_world(_item: TokenStream) -> TokenStream {
    "println!(\"Hello, world!\")".parse().unwrap()
}
