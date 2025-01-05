use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::{IntoTokenStream, val::parse_val},
};

use super::NodeProp;

pub fn parse_flex_basis(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let Some(class) = class.strip_prefix("basis-") else {
        return Ok(false);
    };

    let val = parse_val(class).ok_or(ParseClassError::Unsupported)?;

    ctx.insert_node_prop(NodeProp::FlexBasis, val.into_token_stream());

    Ok(true)
}
