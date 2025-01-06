use crate::{ParseCtx, ParseResult, utils::val::Val};

use super::NodeProp;

pub fn parse_flex(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (flex_grow, flex_shrink, flex_basis) = match class {
        "flex-1" => (1.0, 1.0, Val::Percent(0.)),
        "flex-auto" => (1.0, 1.0, Val::Auto),
        "flex-initial" => (0.0, 1.0, Val::Auto),
        "flex-none" => (0.0, 0.0, Val::Auto),
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::FlexGrow, flex_grow);
    ctx.insert_node_prop_simple(NodeProp::FlexShrink, flex_shrink);
    ctx.insert_node_prop_simple(NodeProp::FlexBasis, flex_basis);
    Ok(true)
}
