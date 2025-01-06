use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::val::{ParseValSettings, Val},
};

use super::NodeProp;

pub fn parse_width(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("w-") {
        return Ok(false);
    }

    let class = &class["w-".len()..];
    let val =
        Val::parse(class, ParseValSettings::default_allow()).ok_or(ParseClassError::Unsupported)?;

    ctx.insert_node_prop_simple(NodeProp::Width, val);

    Ok(true)
}

pub fn parse_min_width(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("min-w-") {
        return Ok(false);
    }

    let class = &class["min-w-".len()..];

    let val =
        Val::parse(class, ParseValSettings::default_allow()).ok_or(ParseClassError::Unsupported)?;

    ctx.insert_node_prop_simple(NodeProp::MinWidth, val);

    Ok(true)
}
