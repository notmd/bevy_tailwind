use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::val::{ParseValSettings, Val},
};

use super::NodeProp;

pub fn parse_flex_basis(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let Some(class) = class.strip_prefix("basis-") else {
        return Ok(false);
    };

    let val =
        Val::parse(class, ParseValSettings::default_allow()).ok_or(ParseClassError::Unknown)?;

    ctx.insert_node_prop_priority(NodeProp::FlexBasis, val, 0);

    Ok(true)
}
