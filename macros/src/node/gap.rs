use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::val::{ParseValSettings, Val},
};

use super::NodeProp;

pub fn parse_gap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let settings = ParseValSettings::default_disallow().allow_px(true);

    if class.starts_with("gap-x-") {
        let class = &class["gap-x-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        ctx.insert_node_prop_simple(NodeProp::RowGap, val);

        return Ok(true);
    }

    if class.starts_with("gap-y-") {
        let class = &class["gap-y-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        ctx.insert_node_prop_simple(NodeProp::ColumnGap, val);

        return Ok(true);
    }

    if class.starts_with("gap-") {
        let class = &class["gap-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        if !ctx.components.node.contains_key(&NodeProp::RowGap) {
            ctx.insert_node_prop_simple(NodeProp::RowGap, val);
        }

        if !ctx.components.node.contains_key(&NodeProp::ColumnGap) {
            ctx.insert_node_prop_simple(NodeProp::ColumnGap, val);
        }

        return Ok(true);
    }

    Ok(false)
}
