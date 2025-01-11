use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_flex_grow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let value = match class {
        "flex-grow" | "grow" => 1.0f32,
        "flex-grow-0" | "grow-0" => 0.0f32,
        _ => return Ok(false),
    };

    ctx.insert_node_prop_priority(NodeProp::FlexGrow, quote! { #value }, 0);
    Ok(true)
}
