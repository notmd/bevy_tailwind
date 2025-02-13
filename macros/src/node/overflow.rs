use crate::{
    picking::{deny_picking_style, insert_picking_style},
    utils::{deny_computed_style, quote::ToTokenStream},
    ParseClassError, ParseCtx, ParseResult,
};
use proc_macro2::TokenStream;
use quote::quote;

use super::NodeProp;

macro_rules! insert_computed_props {
    ($ctx:ident, [$($picking_prop:ident),*], $priority:literal, $props:expr) => {
        match $ctx.class_type.clone() {
            crate::ClassType::Computed(expr) => {
                if $ctx.hover || $ctx.focus {
                    $(
                        $ctx.insert_picking_style(
                            crate::picking::PickingStyleProp::$picking_prop,
                            expr.clone(),
                        );
                    )*
                } else {
                    crate::node::insert_node_prop_nested!(
                        $ctx,
                        NodeProp::Overflow,
                        quote::quote! {bevy::ui::Overflow},
                        expr.clone(),
                        $priority,
                        $props
                    );
                }
                return Ok(true);
            }
            _ => {}
        }
    };
}

pub fn parse_overflow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "overflow" => {
            deny_picking_style!(ctx);
            insert_computed_props!(ctx, [OverflowX, OverflowY], 0, ["x", "y"]);
        }
        "overflow-x" => {
            insert_computed_props!(ctx, [OverflowX], 1, ["x"]);
        }
        "overflow-y" => {
            insert_computed_props!(ctx, [OverflowY], 1, ["y"]);
        }
        _ => {}
    }
    if !class.starts_with("overflow-") {
        return Ok(false);
    }

    let class = &class["overflow-".len()..];

    macro_rules! insert_props {
        ($ctx:ident, $node_prop:expr, $value:expr, $priority:literal, $props:expr) => {
            crate::node::insert_node_prop_nested!(
                $ctx,
                $node_prop,
                quote::quote! {bevy::ui::Overflow},
                $value,
                $priority,
                $props
            );
        };
    }

    if let Ok(axis) = parse_axis(class) {
        deny_computed_style!(ctx);
        deny_picking_style!(ctx);
        insert_props!(ctx, NodeProp::Overflow, axis, 0, ["x", "y"]);
    }

    if class.starts_with("x-") {
        let class = &class["x-".len()..];
        let axis = parse_axis(class)?;
        deny_computed_style!(ctx);
        insert_picking_style!(ctx, OverflowX, axis);
        insert_props!(ctx, NodeProp::Overflow, axis, 1, ["x"]);
    }

    if class.starts_with("y-") {
        let class = &class["y-".len()..];
        let axis = parse_axis(class)?;
        deny_computed_style!(ctx);
        insert_picking_style!(ctx, OverflowY, axis);
        insert_props!(ctx, NodeProp::Overflow, axis, 1, ["y"]);
    }

    Ok(false)
}

fn parse_axis(class: &str) -> Result<OverflowAxis, ParseClassError> {
    let axis = match class {
        "visible" => OverflowAxis::Visible,
        "clip" => OverflowAxis::Clip,
        "hidden" => OverflowAxis::Hidden,
        "scroll" => OverflowAxis::Scroll,
        _ => return Err(ParseClassError::Unknown),
    };

    Ok(axis)
}

#[derive(Clone, Copy)]
enum OverflowAxis {
    Visible,
    Clip,
    Hidden,
    Scroll,
}

impl ToTokenStream for OverflowAxis {
    fn to_token_stream(&self) -> TokenStream {
        match self {
            OverflowAxis::Visible => quote! { bevy::ui::OverflowAxis::Visible },
            OverflowAxis::Clip => quote! { bevy::ui::OverflowAxis::Clip },
            OverflowAxis::Hidden => quote! { bevy::ui::OverflowAxis::Hidden },
            OverflowAxis::Scroll => quote! { bevy::ui::OverflowAxis::Scroll },
        }
    }
}
