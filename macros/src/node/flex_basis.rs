use crate::{
    picking::insert_picking_style,
    utils::{
        insert_computed_style,
        val::{ParseValSettings, Val},
    },
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_flex_basis(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "basis" {
        insert_computed_style!(ctx, node, FlexBasis, NodeProp::FlexBasis, 0);
    }

    let Some(class) = class.strip_prefix("basis-") else {
        return Ok(false);
    };

    let val =
        Val::parse(class, ParseValSettings::default_allow()).ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, FlexBasis, val);
    ctx.insert_node_prop_priority(NodeProp::FlexBasis, val, 0);

    Ok(true)
}
