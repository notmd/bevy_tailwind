use crate::{
    utils::val::{ParseValSettings, Val},
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_gap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let settings = ParseValSettings::default_disallow().allow_px(true);

    if class.starts_with("gap-x-") {
        let class = &class["gap-x-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        ctx.insert_node_prop_priority(NodeProp::RowGap, val, 1);

        return Ok(true);
    }

    if class.starts_with("gap-y-") {
        let class = &class["gap-y-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        ctx.insert_node_prop_priority(NodeProp::ColumnGap, val, 1);

        return Ok(true);
    }

    if class.starts_with("gap-") {
        let class = &class["gap-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        ctx.insert_node_prop(NodeProp::RowGap, val);
        ctx.insert_node_prop(NodeProp::ColumnGap, val);

        return Ok(true);
    }

    Ok(false)
}
