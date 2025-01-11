use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_shrink(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let value = match class {
        "flex-shrink" | "shrink" => 1.0f32,
        "flex-shrink-0" | "shrink-0" => 0.0f32,
        _ => return Ok(false),
    };

    ctx.insert_node_prop_priority(NodeProp::FlexShrink, quote! { #value }, 0);
    Ok(true)
}
