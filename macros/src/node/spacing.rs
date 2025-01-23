use crate::{
    node::insert_node_ui_rect,
    picking::{deny_picking_style, insert_picking_style},
    utils::{
        parse_neg,
        val::{ParseValSettings, Val},
    },
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_padding(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (neg, class) = parse_neg(class);

    if class.starts_with("pt-") {
        let val = parse_padding_val(class, neg)?;
        insert_picking_style!(ctx, PaddingTop, val);
        insert_node_ui_rect!(ctx, NodeProp::Padding, val, 2, ["top"]);
    }

    if class.starts_with("pr-") {
        let val = parse_padding_val(class, neg)?;
        insert_picking_style!(ctx, PaddingRight, val);
        insert_node_ui_rect!(ctx, NodeProp::Padding, val, 2, ["right"]);
    }

    if class.starts_with("pb-") {
        let val = parse_padding_val(class, neg)?;
        insert_picking_style!(ctx, PaddingBottom, val);
        insert_node_ui_rect!(ctx, NodeProp::Padding, val, 2, ["bottom"]);
    }

    if class.starts_with("pl-") {
        let val = parse_padding_val(class, neg)?;
        insert_picking_style!(ctx, PaddingLeft, val);
        insert_node_ui_rect!(ctx, NodeProp::Padding, val, 2, ["left"]);
    }

    if class.starts_with("px-") {
        let val = parse_padding_val(class, neg)?;
        deny_picking_style!(ctx);
        insert_node_ui_rect!(ctx, NodeProp::Padding, val, 1, ["left", "right"]);
    }

    if class.starts_with("py-") {
        let val = parse_padding_val(class, neg)?;
        deny_picking_style!(ctx);
        insert_node_ui_rect!(ctx, NodeProp::Padding, val, 1, ["top", "bottom"]);
    }

    if class.starts_with("p-") {
        let val = parse_padding_val(class, neg)?;
        deny_picking_style!(ctx);
        insert_node_ui_rect!(
            ctx,
            NodeProp::Padding,
            val,
            0,
            ["top", "right", "bottom", "left"]
        );
    }

    Ok(false)
}

fn parse_padding_val(class: &str, neg: bool) -> Result<Val, ParseClassError> {
    Val::parse(
        if class.starts_with("p-") {
            &class[2..]
        } else {
            &class[3..]
        },
        ParseValSettings::default_disallow().allow_px(true),
    )
    .and_then(|val| val.eval_neg(neg))
    .ok_or(ParseClassError::Unknown)
}

pub fn parse_margin(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class.starts_with("mt-") {
        let val = parse_margin_val(class)?;
        insert_picking_style!(ctx, MarginTop, val);
        insert_node_ui_rect!(ctx, NodeProp::Margin, val, 2, ["top"]);
    }

    if class.starts_with("mr-") {
        let val = parse_margin_val(class)?;
        insert_picking_style!(ctx, MarginRight, val);
        insert_node_ui_rect!(ctx, NodeProp::Margin, val, 2, ["right"]);
    }

    if class.starts_with("mb-") {
        let val = parse_margin_val(class)?;
        insert_picking_style!(ctx, MarginBottom, val);
        insert_node_ui_rect!(ctx, NodeProp::Margin, val, 2, ["bottom"]);
    }

    if class.starts_with("ml-") {
        let val = parse_margin_val(class)?;
        insert_picking_style!(ctx, MarginLeft, val);
        insert_node_ui_rect!(ctx, NodeProp::Margin, val, 2, ["left"]);
    }

    if class.starts_with("mx-") {
        let val = parse_margin_val(class)?;
        deny_picking_style!(ctx);
        insert_node_ui_rect!(ctx, NodeProp::Margin, val, 1, ["left", "right"]);
    }

    if class.starts_with("my-") {
        let val = parse_margin_val(class)?;
        deny_picking_style!(ctx);
        insert_node_ui_rect!(ctx, NodeProp::Margin, val, 1, ["top", "bottom"]);
    }

    if class.starts_with("m-") {
        let val = parse_margin_val(class)?;
        deny_picking_style!(ctx);
        insert_node_ui_rect!(
            ctx,
            NodeProp::Margin,
            val,
            0,
            ["top", "right", "bottom", "left"]
        );
    }

    Ok(false)
}

fn parse_margin_val(class: &str) -> Result<Val, ParseClassError> {
    Val::parse(
        if class.starts_with("m-") {
            &class[2..]
        } else {
            &class[3..]
        },
        ParseValSettings::default_disallow().allow_px(true),
    )
    .ok_or(ParseClassError::Unknown)
}
