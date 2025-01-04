mod background_color;
mod node;
mod utils;
mod z_index;

use std::collections::HashMap;

use node::NodeProp;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, LitStr, Token, parse::Parse, parse_macro_input, punctuated::Punctuated};

enum InputElement {
    String(LitStr),
    Object((LitStr, Expr)),
}

impl Parse for InputElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let lit = input.parse()?;
            Ok(InputElement::String(lit))
        } else if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let key: LitStr = content.parse()?;
            content.parse::<Token![:]>()?;
            let value: Expr = content.parse()?;
            Ok(InputElement::Object((key, value)))
        } else {
            Err(input.error("Expected a string literal or a JSON-like object"))
        }
    }
}

struct Input {
    elements: Punctuated<InputElement, Token![,]>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elements = Punctuated::<InputElement, Token![,]>::parse_separated_nonempty(input)?;

        return Ok(Input { elements });
    }
}

macro_rules! parse_class {
    ($class:ident, $span:ident, $($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    continue;
                }
                Err(e) => {
                    let msg = match e {
                        ParseClassError::Unsupported => {
                            format!("Unsuported class: {}", $class)
                        }
                        ParseClassError::Conflict => {
                            format!("Conflict class: {}", $class)
                        }
                    };
                    return syn::Error::new($span, msg)
                        .to_compile_error()
                        .into();
                }
                _ => {}
            }
        )*
    };
}

#[proc_macro]
pub fn tw(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: Input = parse_macro_input!(input);
    let mut ctx = ParseCtx::default();

    for element in input.elements.iter() {
        match element {
            InputElement::String(classes) => {
                let span = classes.span();
                for class in classes.value().split_whitespace() {
                    parse_class!(
                        class,
                        span,
                        ctx.parse_background_color_class(class),
                        ctx.parse_z_index(class),
                        ctx.parse_node_class(class)
                    );

                    return syn::Error::new(
                        classes.span(),
                        format!("Unsuported class:  {}", class),
                    )
                    .to_compile_error()
                    .into();
                }
            }
            InputElement::Object(_) => {}
        }
    }

    let components: Vec<Option<TokenStream>> = vec![
        ctx.get_node(),
        ctx.get_background_color(),
        ctx.get_z_index(),
    ]
    .into_iter()
    .filter(Option::is_some)
    .collect();

    let inner = quote! {
        ( #(#components),* )
    };

    if components.len() == 1 {
        return inner.into();
    }

    let bundle = quote! {
        ( #inner )
    };

    return bundle.into();
}

#[derive(Default)]
struct ParseCtx {
    node_props: HashMap<NodeProp, TokenStream>,
    background_color: Option<TokenStream>,
    z_index: Option<TokenStream>,
}

enum ParseClassError {
    Unsupported,
    Conflict,
}

type ParseResult = Result<bool, ParseClassError>;
