use crate::{
    picking::insert_picking_style,
    utils::{
        insert_computed_style,
        val::{ParseValSettings, Val},
    },
    ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_trbl(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "top" => {
            insert_computed_style!(ctx, node, Top, NodeProp::Top, 1);
        }
        "right" => {
            insert_computed_style!(ctx, node, Right, NodeProp::Right, 1);
        }
        "bottom" => {
            insert_computed_style!(ctx, node, Bottom, NodeProp::Bottom, 1);
        }
        "left" => {
            insert_computed_style!(ctx, node, Left, NodeProp::Left, 1);
        }
        _ => {}
    }

    let Some((prefix, val)) = class.split_once("-") else {
        return Ok(false);
    };

    let Some(val) = Val::parse(
        val,
        ParseValSettings::default_disallow()
            .allow_px(true)
            .allow_fraction(true)
            .allow_auto(true)
            .allow_px(true)
            .allow_full(true),
    ) else {
        return Ok(false);
    };

    match prefix {
        "top" => {
            insert_picking_style!(ctx, Top, val);
            ctx.insert_node_prop_priority(NodeProp::Top, val, 1);
            Ok(true)
        }
        "right" => {
            insert_picking_style!(ctx, Right, val);
            ctx.insert_node_prop_priority(NodeProp::Right, val, 1);
            Ok(true)
        }
        "bottom" => {
            insert_picking_style!(ctx, Bottom, val);
            ctx.insert_node_prop_priority(NodeProp::Bottom, val, 1);
            Ok(true)
        }
        "left" => {
            insert_picking_style!(ctx, Left, val);
            ctx.insert_node_prop_priority(NodeProp::Left, val, 1);
            Ok(true)
        }
        // TODO parse inset* class
        _ => Ok(false),
    }
}
