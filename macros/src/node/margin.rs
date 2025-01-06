use crate::{
    ParseClassError, ParseCtx, ParseResult,
    node::UiRect,
    utils::{
        StructPropValue,
        val::{ParseValSettings, Val},
    },
};

use super::NodeProp;

pub fn parse_margin(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class.starts_with("mt-") {
        let val = parse_val(class)?;
        ctx.components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .top = Some(val);

        return Ok(true);
    }

    if class.starts_with("mr-") {
        let val = parse_val(class)?;
        ctx.components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .right = Some(val);

        return Ok(true);
    }

    if class.starts_with("mb-") {
        let val = parse_val(class)?;
        ctx.components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .bottom = Some(val);

        return Ok(true);
    }

    if class.starts_with("ml-") {
        let val = parse_val(class)?;
        ctx.components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .left = Some(val);

        return Ok(true);
    }

    if class.starts_with("mx-") {
        let val = parse_val(class)?;
        let rect = ctx
            .components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>();

        if rect.right.is_none() {
            rect.right = Some(val);
        }
        if rect.left.is_none() {
            rect.left = Some(val);
        }

        return Ok(true);
    }

    if class.starts_with("my-") {
        let val = parse_val(class)?;
        let rect = ctx
            .components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>();

        if rect.top.is_none() {
            rect.top = Some(val);
        }
        if rect.bottom.is_none() {
            rect.bottom = Some(val);
        }

        return Ok(true);
    }

    if class.starts_with("m-") {
        let val = parse_val(class)?;
        let rect = ctx
            .components
            .node
            .entry(NodeProp::Margin)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>();

        if rect.top.is_none() {
            rect.top = Some(val);
        }
        if rect.right.is_none() {
            rect.right = Some(val);
        }
        if rect.bottom.is_none() {
            rect.bottom = Some(val);
        }
        if rect.left.is_none() {
            rect.left = Some(val);
        }

        return Ok(true);
    }

    Ok(false)
}

fn parse_val(class: &str) -> Result<Val, ParseClassError> {
    Val::parse(
        if class.starts_with("m-") {
            &class[2..]
        } else {
            &class[3..]
        },
        ParseValSettings::default_disallow().allow_px(true),
    )
    .ok_or(ParseClassError::Unsupported)
}
