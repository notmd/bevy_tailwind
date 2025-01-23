use crate::{
    picking::{deny_picking_style, insert_picking_style},
    utils::val::{ParseValSettings, Val},
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_width(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("w-") {
        return Ok(false);
    }

    let class = &class["w-".len()..];
    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_auto(true)
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true)
            .allow_screen_width(true)
            .allow_dimension_screen_width(true),
    )
    .ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, Width, val);
    ctx.insert_node_prop_priority(NodeProp::Width, val, 1);

    Ok(true)
}

pub fn parse_min_width(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("min-w-") {
        return Ok(false);
    }

    let class = &class["min-w-".len()..];

    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true)
            .allow_dimension_screen_width(false),
    )
    .ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, MinWidth, val);
    ctx.insert_node_prop(NodeProp::MinWidth, val);

    Ok(true)
}

pub fn parse_max_width(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("max-w-") {
        return Ok(false);
    }

    let class = &class["max-w-".len()..];

    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true),
    )
    .ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, MaxWidth, val);
    ctx.insert_node_prop(NodeProp::MaxWidth, val);

    Ok(true)
}

pub fn parse_height(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("h-") {
        return Ok(false);
    }

    let class = &class["h-".len()..];
    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_auto(true)
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true)
            .allow_screen_height(true)
            .allow_dimension_screen_height(true),
    )
    .ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, Height, val);
    ctx.insert_node_prop_priority(NodeProp::Height, val, 1);

    Ok(true)
}

pub fn parse_min_height(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("min-h-") {
        return Ok(false);
    }

    let class = &class["min-h-".len()..];

    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true),
    )
    .ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, MinHeight, val);
    ctx.insert_node_prop(NodeProp::MinHeight, val);

    Ok(true)
}

pub fn parse_max_height(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("max-h-") {
        return Ok(false);
    }

    let class = &class["max-h-".len()..];

    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true),
    )
    .ok_or(ParseClassError::Unknown)?;

    insert_picking_style!(ctx, MaxHeight, val);
    ctx.insert_node_prop(NodeProp::MaxHeight, val);

    Ok(true)
}

pub fn parse_size(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("size-") {
        return Ok(false);
    }

    let class = &class["size-".len()..];

    let val = Val::parse(
        class,
        ParseValSettings::default_disallow()
            .allow_auto(true)
            .allow_px(true)
            .allow_fraction(true)
            .allow_full(true),
    )
    .ok_or(ParseClassError::Unknown)?;

    deny_picking_style!(ctx);
    ctx.insert_node_prop_priority(NodeProp::Width, val, 0);
    ctx.insert_node_prop_priority(NodeProp::Height, val, 0);

    Ok(true)
}
