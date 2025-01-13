mod background;
mod border;
mod node;
mod outline;
mod text;
mod utils;
mod z_index;

use node::NodeProp;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned, Expr, LitStr, Token,
};
use utils::quote::{Quote, QuoteCtx, Struct};

macro_rules! parse_class {
    ($class:ident, $span:ident, $($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    continue;
                }
                Err(e) => {
                    let msg = match e {
                        ParseClassError::Unknown => {
                            format!("Unknown class: {}", $class)
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
                $ctx.parse_outline(class),
                $ctx.parse_background(class),
                $ctx.parse_text(class),
                $ctx.parse_node(class)
            );

            return syn::Error::new(span, format!("Unknown class:  {}", class))
                .to_compile_error()
                .into();
        }
    };
}

/// This macro allows you to create or mutate bevy UI component(s) with `TailwindCSS` classes and `clsx` object syntax.
/// To create components, you can pass a string of `TailwindCSS` classes or an object of `TailwindCSS` classes.
/// ```rust,ignore
/// use bevy_tailwind::tw;
///
/// let bundle: (BackgroundColor, Node) = tw!("bg-white w-full h-full");
/// let bundle: (BackgroundColor, Node) = tw!("h-full w-full", {
///    "bg-white p-6": true,
/// })
/// ```
///
/// To mutate components, you can pass an expression as the first argument. The expression either returns an owned value or a mutable reference to the component. You can only one component at a time. The macro will return whatever the expression returns.
/// ```rust,ignore
/// use bevy_tailwind::tw;
///
/// let node: Node = tw!(get_node(), "w-full h-full", {
///   "p-6": true,
/// });
///
/// let node: &mut Node = tw!(&mut node, "m-10");
/// ```
///
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

    let mut qctx = QuoteCtx {
        condition_idents: &condition_idents,
        is_create: matches!(ctx.macro_type, MacroType::Create),
        parent_props: vec!["__comp".to_string()],
    };

    let conditions = &ctx.conditions;
    let condition = quote! {
        #(#conditions)*
    };

    let components = [
        (
            ctx.components.node.quote(&mut qctx),
            ctx.components.node.path(),
        ),
        (
            ctx.components.background_color.quote(&mut qctx),
            ctx.components.background_color.path(),
        ),
        (
            ctx.components.z_index.quote(&mut qctx),
            ctx.components.z_index.path(),
        ),
        (
            ctx.components.text_font.quote(&mut qctx),
            ctx.components.text_font.path(),
        ),
        (
            ctx.components.text_layout.quote(&mut qctx),
            ctx.components.text_layout.path(),
        ),
        (
            ctx.components.text_color.quote(&mut qctx),
            ctx.components.text_color.path(),
        ),
        (
            ctx.components.border_radius.quote(&mut qctx),
            ctx.components.border_radius.path(),
        ),
        (
            ctx.components.border_color.quote(&mut qctx),
            ctx.components.border_color.path(),
        ),
        (
            ctx.components.outline.quote(&mut qctx),
            ctx.components.outline.path(),
        ),
    ]
    .into_iter()
    .filter(|component| !component.0.is_empty());

    match ctx.macro_type {
        MacroType::Create => {
            let components = components.map(|(component, _)| component);
            return quote! {
                {
                    #condition
                    ( #(#components),* )
                }
            }
            .into();
        }
        MacroType::Mutate(expr) => {
            // if components.len() > 1 {
            //     return syn::Error::new(
            //         Span::call_site(),
            //         "Mutation syntax does not support mutate multiple components",
            //     )
            //     .to_compile_error()
            //     .into();
            // }

            let components = components.into_iter().map(|(stmts, path)| {
                quote! {
                    if let Some(mut __comp) = __entity.get_mut::<#path>() {
                        #stmts
                    }
                }
            });

            return quote! {
                {
                    let mut __entity = #expr;
                    #condition
                    #(#components)*
                    __entity
                }
            }
            .into();
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

struct UiComponents {
    node: Struct<NodeProp>,
    background_color: Struct<&'static str>,
    z_index: Struct<&'static str>,
    text_font: Struct<&'static str>,
    text_layout: Struct<&'static str>,
    text_color: Struct<&'static str>,
    border_radius: Struct<&'static str>,
    border_color: Struct<&'static str>,
    outline: Struct<&'static str>,
}

impl Default for UiComponents {
    fn default() -> Self {
        Self {
            node: Struct::new(quote! { bevy::ui::Node }),
            background_color: Struct::new(quote! { bevy::ui::BackgroundColor }),
            z_index: Struct::new(quote! { bevy::ui::ZIndex }),
            text_font: Struct::new(quote! { bevy::text::TextFont }),
            text_layout: Struct::new(quote! { bevy::text::TextLayout }),
            text_color: Struct::new(quote! { bevy::text::TextColor }),
            border_radius: Struct::new(quote! { bevy::ui::BorderRadius }),
            border_color: Struct::new(quote! { bevy::ui::BorderColor }),
            outline: Struct::new(quote! { bevy::ui::Outline }),
        }
    }
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
    Unknown,
}

type ParseResult = Result<bool, ParseClassError>;
