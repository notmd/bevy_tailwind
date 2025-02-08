use crate::{picking::insert_picking_style, utils::insert_computed_style, ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_flex_grow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "flex-grow" || class == "grow" {
        insert_computed_style!(ctx, node, FlexGrow, NodeProp::FlexGrow, 0);
    }

    let value = match class {
        "flex-grow" | "grow" => 1.0f32,
        "flex-grow-0" | "grow-0" => 0.0f32,
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, FlexGrow, quote! { #value });
    ctx.insert_node_prop_priority(NodeProp::FlexGrow, quote! { #value }, 0);
    Ok(true)
}
