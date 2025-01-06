use crate::{
    ParseCtx, ParseResult,
    utils::val::Val,
};

use super::NodeProp;

pub fn parse_flex(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "flex-1" => {
            ctx.insert_node_prop_simple(NodeProp::FlexGrow, 1.0);
            ctx.insert_node_prop_simple(NodeProp::FlexShrink, 1.0);
            ctx.insert_node_prop_simple(NodeProp::FlexBasis, Val::Percent(0.));
            Ok(true)
        }
        "flex-auto" => {
            ctx.insert_node_prop_simple(NodeProp::FlexGrow, 1.0);
            ctx.insert_node_prop_simple(NodeProp::FlexShrink, 1.0);
            ctx.insert_node_prop_simple(NodeProp::FlexBasis, Val::Auto);
            Ok(true)
        }
        "flex-initial" => {
            ctx.insert_node_prop_simple(NodeProp::FlexGrow, 0.0);
            ctx.insert_node_prop_simple(NodeProp::FlexShrink, 1.0);
            ctx.insert_node_prop_simple(NodeProp::FlexBasis, Val::Auto);
            Ok(true)
        }
        "flex-none" => {
            ctx.insert_node_prop_simple(NodeProp::FlexGrow, 0.0);
            ctx.insert_node_prop_simple(NodeProp::FlexShrink, 0.0);
            ctx.insert_node_prop_simple(NodeProp::FlexBasis, Val::Auto);
            Ok(true)
        }
        _ => Ok(false),
    }
}
