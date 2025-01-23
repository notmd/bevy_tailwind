use quote::quote;

use crate::{picking::insert_picking_style, ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_shrink(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let value = match class {
        "flex-shrink" | "shrink" => 1.0f32,
        "flex-shrink-0" | "shrink-0" => 0.0f32,
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, FlexShrink, quote! { #value });
    ctx.insert_node_prop_priority(NodeProp::FlexShrink, quote! { #value }, 0);
    Ok(true)
}
