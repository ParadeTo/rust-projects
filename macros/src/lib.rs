mod builder;
mod raw_builder;

extern crate proc_macro;
use crate::raw_builder::BuilderContext;
use askama::Template;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", input);
    r#"fn hello() { println!("Hello world!"); }"#.parse().unwrap()
}

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    // eprintln!("{:#?}", input);
    // r#"fn hello() { println!("Hello world!"); }"#.parse().unwrap()
    BuilderContext::render(input).unwrap().parse().unwrap()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", input);
    TokenStream::default()
}
