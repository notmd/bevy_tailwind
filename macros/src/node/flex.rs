use crate::{ParseCtx, ParseResult, utils::val::Val};

use super::NodeProp;
use quote::quote;

pub fn parse_flex(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (flex_grow, flex_shrink, flex_basis) = match class {
        "flex-1" => (1.0f32, 1.0f32, Val::Percent(0.)),
        "flex-auto" => (1.0, 1.0, Val::Auto),
        "flex-initial" => (0.0, 1.0, Val::Auto),
        "flex-none" => (0.0, 0.0, Val::Auto),
        _ => return Ok(false),
    };

    ctx.insert_node_prop(NodeProp::FlexGrow, quote! {#flex_grow});
    ctx.insert_node_prop(NodeProp::FlexShrink, quote! {#flex_shrink});
    ctx.insert_node_prop(NodeProp::FlexBasis, flex_basis);
    Ok(true)
}
