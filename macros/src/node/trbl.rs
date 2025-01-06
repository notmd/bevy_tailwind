use crate::{
    ParseCtx, ParseResult,
    utils::val::{ParseValSettings, Val},
};

use super::NodeProp;

pub fn parse_trbl(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let Some((prefix, val)) = class.split_once("-") else {
        return Ok(false);
    };

    let Some(val) = Val::parse(val, ParseValSettings::default_allow()) else {
        return Ok(false);
    };

    match prefix {
        "top" => {
            ctx.insert_node_prop_simple(NodeProp::Top, val);
            Ok(true)
        }
        "right" => {
            ctx.insert_node_prop_simple(NodeProp::Right, val);
            Ok(true)
        }
        "bottom" => {
            ctx.insert_node_prop_simple(NodeProp::Bottom, val);
            Ok(true)
        }
        "left" => {
            ctx.insert_node_prop_simple(NodeProp::Left, val);
            Ok(true)
        }
        _ => Ok(false),
    }
}
