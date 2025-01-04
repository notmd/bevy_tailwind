mod background_color;
mod node;
mod val;

use std::collections::HashMap;

use node::NodeProp;
use proc_macro2::{Span, TokenStream};
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
    ($class:ident, $($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    continue;
                }
                Err(_) => {
                    return syn::Error::new(Span::call_site(), format!("Unsuported class: {}", $class))
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
    let mut ctx = ParseCtx {
        node_props: HashMap::new(),
        background_color: None,
    };

    for element in input.elements.iter() {
        match element {
            InputElement::String(classes) => {
                for class in classes.value().split_whitespace() {
                    parse_class!(
                        class,
                        ctx.parse_node_class(class),
                        ctx.parse_background_color_class(class)
                    );

                    return syn::Error::new(
                        Span::call_site(),
                        format!("Unsuported class:  {}", class),
                    )
                    .to_compile_error()
                    .into();
                }
            }
            InputElement::Object(_) => {}
        }
    }

    let components: Vec<Option<TokenStream>> = vec![ctx.get_node(), ctx.get_background_color()]
        .into_iter()
        .filter(Option::is_some)
        .collect();

    let bundle = quote! {
        ( #(#components)* )
    };

    return bundle.into();
}

#[derive(Default)]
struct ParseCtx {
    node_props: HashMap<NodeProp, TokenStream>,
    background_color: Option<TokenStream>,
}

type ParseResult = Result<bool, ()>;
