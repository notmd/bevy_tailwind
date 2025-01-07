use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::{
        PrioritizedStructPropValue, StructPropValue, parse_neg,
        ui_rect::UiRect,
        val::{ParseValSettings, Val},
    },
};

use super::NodeProp;

macro_rules! insert_ui_rect_props {
    ($ctx:ident, $node_prop:expr, $value:expr, $priority:literal , [$($prop:ident),*]) => {
        $(
            let prop = &mut $ctx.components
                .node
                .entry($node_prop)
                .or_insert_with(|| StructPropValue::nested($ctx.class_type, UiRect::default()))
                .value
                .downcast_mut::<UiRect>()
                .$prop;

            if let Some(prop) = prop.as_mut() {
                prop.set_if_gte_priority($value, $priority);
            } else {
                prop.replace(PrioritizedStructPropValue::new($value, $priority));
            }
        )*

        return Ok(true);
    };
}

pub fn parse_padding(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (neg, class) = parse_neg(class);

    let mut prop: Option<PrioritizedStructPropValue<Val>> = None;

    if let Some(prop) = prop.as_mut() {
        prop.set_if_gte_priority(Val::Auto, 1);
    } else {
        prop.replace(PrioritizedStructPropValue::new(Val::Auto, 1));
    }

    if class.starts_with("pt-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 2, [top]);
    }

    if class.starts_with("pr-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 2, [right]);
    }

    if class.starts_with("pb-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 2, [bottom]);
    }

    if class.starts_with("pl-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 2, [left]);
    }

    if class.starts_with("px-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 1, [left, right]);
    }

    if class.starts_with("py-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 1, [top, bottom]);
    }

    if class.starts_with("p-") {
        let val = parse_padding_val(class, neg)?;
        insert_ui_rect_props!(ctx, NodeProp::Padding, val, 0, [top, right, bottom, left]);
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
    .ok_or(ParseClassError::Unsupported)
}

pub fn parse_margin(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class.starts_with("mt-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 2, [top]);
    }

    if class.starts_with("mr-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 2, [right]);
    }

    if class.starts_with("mb-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 2, [bottom]);
    }

    if class.starts_with("ml-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 2, [left]);
    }

    if class.starts_with("mx-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 1, [left, right]);
    }

    if class.starts_with("my-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 1, [top, bottom]);
    }

    if class.starts_with("m-") {
        let val = parse_margin_val(class)?;
        insert_ui_rect_props!(ctx, NodeProp::Margin, val, 0, [top, right, bottom, left]);
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
    .ok_or(ParseClassError::Unsupported)
}
