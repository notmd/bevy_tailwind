use quote::quote;

use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
    ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_flex_shrink(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "flex-shrink" || class == "shrink" {
        insert_computed_style!(ctx, node, FlexShrink, NodeProp::FlexShrink, 0);
    }

    let value = match class {
        "flex-shrink" | "shrink" => 1.0f32,
        "flex-shrink-0" | "shrink-0" => 0.0f32,
        _ => return Ok(false),
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, FlexShrink, quote! { #value });
    ctx.insert_node_prop_priority(NodeProp::FlexShrink, quote! { #value }, 0);
    Ok(true)
}
