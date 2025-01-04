pub mod val;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub fn parse_neg(str: &str) -> (bool, &str) {
    let neg = str.starts_with('-');
    let str = if neg { &str[1..] } else { str };
    (neg, str)
}

pub trait IntoTokenStream {
    fn into_token_stream(self) -> proc_macro2::TokenStream;
}
