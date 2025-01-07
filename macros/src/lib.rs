mod background;
mod border;
mod node;
mod text;
mod utils;
mod z_index;

use node::NodeProp;
use proc_macro2::{Span, TokenStream};
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
                $ctx.parse_border_radius(class),
                $ctx.parse_border_color(class),
                $ctx.parse_background(class),
                $ctx.parse_text(class),
                $ctx.parse_node(class)
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
                    let idx = ctx.conditions.len();
                    let ident = format_ident!("__class__cond_{}", idx);

                    ctx.conditions.push(quote! {
                        let #ident = #expr;
                    });
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

    let condition_idents = ctx
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

    let components = [
        ctx.components.node.quote(
            quote! { bevy::ui::Node },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.background_color.quote(
            quote! { bevy::ui::BackgroundColor },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.z_index.quote(
            quote! { bevy::ui::ZIndex },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.text_font.quote(
            quote! { bevy::text::TextFont },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.text_layouut.quote(
            quote! { bevy::text::TextLayout },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.text_color.quote(
            quote! { bevy::text::TextColor },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.border_radius.quote(
            quote! { bevy::ui::BorderRadius },
            &condition_idents,
            &ctx.macro_type,
        ),
        ctx.components.border_color.quote(
            quote! { bevy::ui::BorderColor },
            &condition_idents,
            &ctx.macro_type,
        ),
    ]
    .into_iter()
    .filter(Option::is_some)
    .collect::<Vec<_>>();

    let conditions = &ctx.conditions;
    let condition = quote! {
        #(#conditions)*
    };

    let inner = quote! {
         #(#components),*
    };

    if is_mutate {
        if components.len() > 1 {
            return syn::Error::new(
                Span::call_site(),
                "Mutation syntax does not support mutate multiple components",
            )
            .to_compile_error()
            .into();
        }

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
    background_color: StructProps<&'static str>,
    z_index: StructProps<&'static str>,
    text_font: StructProps<&'static str>,
    text_layouut: StructProps<&'static str>,
    text_color: StructProps<&'static str>,
    border_radius: StructProps<&'static str>,
    border_color: StructProps<&'static str>,
}

#[derive(Default)]
struct ParseCtx {
    macro_type: MacroType,
    class_type: ClassType,
    conditions: Vec<TokenStream>,
    components: UiComponents,
}

impl ParseCtx {
    fn new(macro_type: MacroType) -> Self {
        Self {
            macro_type,
            ..Default::default()
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
