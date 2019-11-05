extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn hw(_: TokenStream) -> TokenStream {
    r#"println!("Hello, World!");"#.parse().unwrap()
}