mod background;
mod border;
mod node;
mod outline;
mod picking;
mod text;
mod transform;
mod utils;
mod z_index;

use node::NodeProp;
use picking::PickingStyles;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Expr, LitStr, Token, parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};
use utils::quote::{Quote, QuoteCtx, Struct};

macro_rules! parse_class {
    ($ctx:ident, $class:ident, $span:ident, $($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    continue;
                }
                Err(e) => {
                    let msg = match e {
                        ParseClassError::Unknown => {
                            let prefix = if $ctx.hover {
                                "hover:"
                            } else if $ctx.focus {
                                "focus:"
                            } else {
                                ""
                            };
                            format!("Unknown class: {}{}", prefix, $class)
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
    ($ctx:ident, $classes:ident) => {
        let span = $classes.span();
        for original_class in $classes.value().split_whitespace() {
            let (hover, focus, class) = picking::parse_picking_class(original_class);
            $ctx.hover = hover;
            $ctx.focus = focus;
            parse_class!(
                $ctx,
                class,
                span,
                $ctx.parse_z_index(class),
                $ctx.parse_border_radius(class),
                $ctx.parse_border_color(class),
                $ctx.parse_outline(class),
                $ctx.parse_background(class),
                $ctx.parse_text(class),
                $ctx.parse_transform(class),
                $ctx.parse_node(class)
            );

            return syn::Error::new(span, format!("Unknown class:  {}", original_class))
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
/// To mutate component, you can pass an expression as the first argument. The expression either returns an owned value or a mutable reference to the component. You can only one component at a time. The macro will return whatever the expression returns.
/// ```rust,ignore
/// use bevy_tailwind::tw;
///
/// let node: Node = tw!(get_node(), "w-full h-full", {
///   "p-6": true,
/// });
///
/// let node: &mut Node = tw!(&mut node, "m-10");
/// ```
/// To mutate multiple components, you can use the entity syntax. The syntax is same as component mutation but with the leading `@` symbol and the expression must return `EntityMut`.
/// ```rust,ignore
/// use bevy_tailwind::tw;
///
/// fn my_system(mut query: Query<EntityMut, With<Node>>) {
///     for mut entity in query.iter_mut() {
///       tw!(@ &mut entity, "p-1");
///    }
/// }

#[proc_macro]
pub fn tw(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: Input = parse_macro_input!(input);
    let first = &input.elements[0];

    let is_mutate = matches!(first, InputElement::Mutate(_, _));

    let macro_type = match first {
        InputElement::Mutate(expr, is_entity) => MacroType::Mutate(expr.clone(), *is_entity),
        _ => MacroType::Create,
    };

    let mut ctx = ParseCtx::new(macro_type);

    let skip = if is_mutate { 1 } else { 0 };
    for element in input.elements.into_iter().skip(skip) {
        match element {
            InputElement::String(classes) => {
                ctx.class_type = ClassType::String;
                parse_classes!(ctx, classes);
            }
            InputElement::Object(exprs) => {
                for (classes, expr) in exprs {
                    let idx = ctx.conditions.len();
                    let ident = format_ident!("__class__cond_{}", idx);

                    ctx.conditions.push(quote! {
                        let #ident = #expr;
                    });
                    ctx.class_type = ClassType::Object(ctx.conditions.len() - 1);
                    parse_classes!(ctx, classes);
                }
            }
            InputElement::Computed(classes, expr) => {
                ctx.class_type = ClassType::Computed(expr);
                parse_classes!(ctx, classes);
            }
            InputElement::Mutate(expr, _) => {
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
        parse_ctx: &ctx,
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
            ctx.components.border_color.quote(&mut qctx),
            ctx.components.border_color.path(),
        ),
        (
            ctx.components.outline.quote(&mut qctx),
            ctx.components.outline.path(),
        ),
        (
            ctx.components.transform.quote(&mut qctx),
            ctx.components.transform.path(),
        ),
        (
            ctx.components.picking_styles.quote(&mut qctx),
            ctx.components.picking_styles.path(),
        ),
    ]
    .into_iter()
    .filter(|component| !component.0.is_empty())
    .collect::<Vec<_>>();

    match ctx.macro_type {
        MacroType::Create => {
            let components = components.into_iter().map(|(component, _)| component);
            quote! {
                {
                    #condition
                    #[allow(clippy::needless_update)]
                    ( #(#components),* )
                }
            }
            .into()
        }
        MacroType::Mutate(expr, is_entity) => {
            if !is_entity && components.len() > 1 {
                return syn::Error::new(
                    Span::call_site(),
                    "Mutation syntax does not support mutate multiple components",
                )
                .to_compile_error()
                .into();
            }

            if !is_entity {
                let components = components.into_iter().map(|(component, _)| component);
                return quote! {
                    {
                        let mut __comp = #expr;
                        #condition
                        #(#components)*
                        __comp
                    }
                }
                .into();
            }

            let components = components.into_iter().map(|(stmts, path)| {
                quote! {
                    if let Some(mut __comp) = __entity.get_mut::<#path>() {
                        #stmts
                    }
                }
            });

            quote! {
                {
                    let mut __entity = #expr;
                    #condition
                    #(#components)*
                    __entity
                }
            }
            .into()
        }
    }
}

struct Input {
    elements: Punctuated<InputElement, Token![,]>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elements = Punctuated::<InputElement, Token![,]>::parse_separated_nonempty(input)?;

        Ok(Input { elements })
    }
}

enum InputElement {
    Mutate(Expr, bool), // only first element
    String(LitStr),
    Object(Punctuated<(LitStr, Expr), Token![,]>),
    Computed(LitStr, Expr),
}

impl Parse for InputElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let lit = input.parse()?;
            if input.parse::<Token![:]>().is_ok() {
                let expr: Expr = input.parse()?;

                return Ok(InputElement::Computed(lit, expr));
            }

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
            let is_entity = input.parse::<Token![@]>().is_ok();
            Ok(InputElement::Mutate(input.parse()?, is_entity))
        }
    }
}

#[derive(Default, Clone)]
enum MacroType {
    #[default]
    Create,
    Mutate(Expr, bool),
}

struct UiComponents {
    node: Struct<NodeProp>,
    background_color: Struct<&'static str>,
    z_index: Struct<&'static str>,
    text_font: Struct<&'static str>,
    text_layout: Struct<&'static str>,
    text_color: Struct<&'static str>,
    border_color: Struct<&'static str>,
    outline: Struct<&'static str>,
    transform: Struct<&'static str>,
    picking_styles: PickingStyles,
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
            border_color: Struct::new(quote! { bevy::ui::BorderColor }),
            outline: Struct::new(quote! { bevy::ui::Outline }),
            transform: Struct::new(quote! { bevy::ui::UiTransform }),
            picking_styles: PickingStyles::default(),
        }
    }
}

#[derive(Default)]
struct ParseCtx {
    macro_type: MacroType,
    class_type: ClassType,
    conditions: Vec<TokenStream>,
    components: UiComponents,
    hover: bool,
    focus: bool,
}

impl ParseCtx {
    fn new(macro_type: MacroType) -> Self {
        Self {
            macro_type,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
enum ClassType {
    #[default]
    String,
    Object(usize),
    Computed(Expr),
}

enum ParseClassError {
    Unknown,
}

type ParseResult = Result<bool, ParseClassError>;
