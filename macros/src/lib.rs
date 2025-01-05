mod background_color;
mod node;
mod utils;
mod z_index;

use node::NodeProp;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, LitStr, Token, parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};
use utils::StructProps;

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

macro_rules! parse_classes {
    ($classes:ident, $ctx:ident) => {
        let span = $classes.span();
        for class in $classes.value().split_whitespace() {
            parse_class!(
                class,
                span,
                $ctx.parse_z_index(class),
                $ctx.parse_background_color_class(class),
                $ctx.parse_node_class(class)
            );

            return syn::Error::new(span, format!("Unsuported class:  {}", class))
                .to_compile_error()
                .into();
        }
    };
}

#[proc_macro]
pub fn tw(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: Input = parse_macro_input!(input);
    let first = &input.elements[0];

    let is_mutate = matches!(first, InputElement::Mutate(_));

    let macro_type = match first {
        InputElement::Mutate(expr) => MacroType::Mutate(expr.clone()),
        _ => MacroType::Create,
    };

    let mut ctx = ParseCtx::new(macro_type);

    let skip = if is_mutate { 1 } else { 0 };
    for element in input.elements.into_iter().skip(skip) {
        match element {
            InputElement::String(classes) => {
                ctx.class_type = ClassType::String;
                parse_classes!(classes, ctx);
            }
            InputElement::Object(exprs) => {
                for (classes, expr) in exprs {
                    ctx.conditions.push(expr);
                    ctx.class_type = ClassType::Object(ctx.conditions.len() - 1);
                    parse_classes!(classes, ctx);
                }
            }
            InputElement::Mutate(expr) => {
                return syn::Error::new(expr.span(), "Unexpected expression. Component mutation is only allowed in the first argument")
                    .to_compile_error()
                    .into();
            }
        }
    }

    let conditions_idents = ctx
        .conditions
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let ident = format_ident!("__class__cond_{}", i);
            quote! {
                #ident

            }
        })
        .collect::<Vec<_>>();

    let conditions = ctx
        .conditions
        .iter()
        .enumerate()
        .map(|(i, expr)| {
            let ident = format_ident!("__class__cond_{}", i);

            quote! { let #ident = #expr; }
        })
        .collect::<Vec<_>>();

    let components = vec![
        ctx.components.node.quote(
            quote! { bevy::ui::Node },
            &conditions_idents,
            &ctx.macro_type,
        ),
        ctx.get_background_color(),
        ctx.components.z_index.quote(
            quote! { bevy::ui::ZIndex },
            &conditions_idents,
            &ctx.macro_type,
        ),
    ]
    .into_iter()
    .filter(Option::is_some);

    let condition = conditions.into_iter().collect::<TokenStream>();

    let inner = quote! {
         #(#components),*
    };

    if is_mutate {
        return quote! {
            {
                #condition
                #inner
            }
        }
        .into();
    }

    return quote! {
        {
            #condition
            ( #inner )
        }
    }
    .into();
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

enum InputElement {
    Mutate(Expr), // only first element
    String(LitStr),
    Object(Punctuated<(LitStr, Expr), Token![,]>),
}

impl Parse for InputElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let lit = input.parse()?;
            Ok(InputElement::String(lit))
        } else if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let exprs = Punctuated::<(LitStr, Expr), Token![,]>::parse_separated_nonempty_with(
                &content,
                |p| {
                    let key: LitStr = p.parse()?;
                    p.parse::<Token![:]>()?;
                    let value: Expr = p.parse()?;
                    Ok((key, value))
                },
            )?;

            Ok(InputElement::Object(exprs))
        } else {
            Ok(InputElement::Mutate(input.parse()?))
        }
    }
}

#[derive(Default, Clone)]
enum MacroType {
    #[default]
    Create,
    Mutate(Expr),
}

#[derive(Default)]
struct UiComponents {
    node: StructProps<NodeProp>,
    // background_color: String,
    z_index: StructProps<&'static str>,
}

#[derive(Default)]
struct ParseCtx {
    macro_type: MacroType,
    class_type: ClassType,
    conditions: Vec<Expr>,
    components: UiComponents,
}

impl ParseCtx {
    fn new(macro_type: MacroType) -> Self {
        Self {
            macro_type,
            ..Default::default()
        }
    }

    fn quote_tuple_component(
        &self,
        path: TokenStream,
        props: impl IntoIterator<Item = TokenStream>,
    ) -> TokenStream {
        match &self.macro_type {
            crate::MacroType::Create => {
                let props = props.into_iter().collect::<Vec<_>>();
                return quote! {
                    #path (
                        #(#props),*
                    )
                };
            }
            crate::MacroType::Mutate(expr) => {
                let props = props
                    .into_iter()
                    .enumerate()
                    .map(|(i, prop)| {
                        let i = syn::Index::from(i);
                        quote! {
                            __comp.#i = #prop;
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                   {
                       let __comp = &mut #expr;
                        #(#props)*
                   }
                }
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ClassType {
    #[default]
    String,
    Object(usize),
}

enum ParseClassError {
    Unsupported,
    Conflict,
}

type ParseResult = Result<bool, ParseClassError>;
