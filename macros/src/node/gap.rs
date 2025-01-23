use crate::{
    picking::{deny_picking_style, insert_picking_style},
    utils::val::{ParseValSettings, Val},
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_gap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let settings = ParseValSettings::default_disallow().allow_px(true);

    if class.starts_with("gap-x-") {
        let class = &class["gap-x-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        insert_picking_style!(ctx, ColumnGap, val);
        ctx.insert_node_prop_priority(NodeProp::ColumnGap, val, 1);

        return Ok(true);
    }

    if class.starts_with("gap-y-") {
        let class = &class["gap-y-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        insert_picking_style!(ctx, RowGap, val);
        ctx.insert_node_prop_priority(NodeProp::RowGap, val, 1);

        return Ok(true);
    }

    if class.starts_with("gap-") {
        let class = &class["gap-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        deny_picking_style!(ctx);
        ctx.insert_node_prop(NodeProp::RowGap, val);
        ctx.insert_node_prop(NodeProp::ColumnGap, val);

        return Ok(true);
    }

    Ok(false)
}
