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
            ctx.insert_node_prop_priority(NodeProp::Top, val, 1);
            Ok(true)
        }
        "right" => {
            ctx.insert_node_prop_priority(NodeProp::Right, val, 1);
            Ok(true)
        }
        "bottom" => {
            ctx.insert_node_prop_priority(NodeProp::Bottom, val, 1);
            Ok(true)
        }
        "left" => {
            ctx.insert_node_prop_priority(NodeProp::Left, val, 1);
            Ok(true)
        }
        // TODO parse inset* class
        _ => Ok(false),
    }
}
