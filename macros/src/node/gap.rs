use crate::{
    picking::{deny_picking_style, insert_picking_style},
    utils::{
        deny_computed_style, insert_computed_style,
        val::{ParseValSettings, Val},
    },
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_gap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "gap-x" => {
            insert_computed_style!(ctx, node, ColumnGap, NodeProp::ColumnGap, 1);
        }
        "gap-y" => {
            insert_computed_style!(ctx, node, RowGap, NodeProp::RowGap, 1);
        }
        "gap" => {
            insert_computed_style!(
                ctx,
                node,
                [
                    (RowGap, NodeProp::RowGap, 0),
                    (ColumnGap, NodeProp::ColumnGap, 0)
                ]
            );
        }
        _ => {}
    }

    let settings = ParseValSettings::default_disallow().allow_px(true);

    if class.starts_with("gap-x-") {
        let class = &class["gap-x-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        deny_computed_style!(ctx);
        insert_picking_style!(ctx, ColumnGap, val);
        ctx.insert_node_prop_priority(NodeProp::ColumnGap, val, 1);

        return Ok(true);
    }

    if class.starts_with("gap-y-") {
        let class = &class["gap-y-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        deny_computed_style!(ctx);
        insert_picking_style!(ctx, RowGap, val);
        ctx.insert_node_prop_priority(NodeProp::RowGap, val, 1);

        return Ok(true);
    }

    if class.starts_with("gap-") {
        let class = &class["gap-".len()..];
        let val = Val::parse(class, settings).ok_or(ParseClassError::Unknown)?;

        deny_computed_style!(ctx);
        deny_picking_style!(ctx);
        ctx.insert_node_prop(NodeProp::RowGap, val);
        ctx.insert_node_prop(NodeProp::ColumnGap, val);

        return Ok(true);
    }

    Ok(false)
}
