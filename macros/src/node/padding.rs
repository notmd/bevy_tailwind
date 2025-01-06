use crate::{
    ParseClassError, ParseCtx, ParseResult,
    node::UiRect,
    utils::{
        StructPropValue, parse_neg,
        val::{ParseValSettings, Val},
    },
};

use super::NodeProp;

pub fn parse_padding(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (neg, class) = parse_neg(class);

    if class.starts_with("pt-") {
        let val = parse_val(class, neg)?;
        ctx.components
            .node
            .entry(NodeProp::Padding)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .top = Some(val);

        return Ok(true);
    }

    if class.starts_with("pr-") {
        let val = parse_val(class, neg)?;
        ctx.components
            .node
            .entry(NodeProp::Padding)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .right = Some(val);

        return Ok(true);
    }

    if class.starts_with("pb-") {
        let val = parse_val(class, neg)?;
        ctx.components
            .node
            .entry(NodeProp::Padding)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .bottom = Some(val);

        return Ok(true);
    }

    if class.starts_with("pl-") {
        let val = parse_val(class, neg)?;
        ctx.components
            .node
            .entry(NodeProp::Padding)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, UiRect::default()))
            .value
            .downcast_mut::<UiRect>()
            .left = Some(val);

        return Ok(true);
    }

    if class.starts_with("px-") {
        let val = parse_val(class, neg)?;
        let rect = ctx
            .components
            .node
            .entry(NodeProp::Padding)
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

    if class.starts_with("py-") {
        let val = parse_val(class, neg)?;
        let rect = ctx
            .components
            .node
            .entry(NodeProp::Padding)
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

    if class.starts_with("p-") {
        let val = parse_val(class, neg)?;
        let rect = ctx
            .components
            .node
            .entry(NodeProp::Padding)
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

fn parse_val(class: &str, neg: bool) -> Result<Val, ParseClassError> {
    Val::parse(
        if class.starts_with("p-") {
            &class[2..]
        } else {
            &class[3..]
        },
        ParseValSettings::default_disallow().allow_px(true),
    )
    .and_then(|val| val.eval_neg(neg))
    .ok_or(ParseClassError::Unsupported)
}
