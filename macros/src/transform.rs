use crate::{
    picking::{insert_picking_style, PickingStyleProp},
    utils::{
        deny_computed_style, insert_computed_style, parse_neg,
        quote::ToTokenStream,
        val::{ParseValSettings, Val},
    },
    ParseClassError, ParseCtx, ParseResult,
};

impl ParseCtx {
    pub fn parse_transform(&mut self, class: &str) -> ParseResult {
        if parse_translate(self, class)? {
            return Ok(true);
        }
        if parse_scale(self, class)? {
            return Ok(true);
        }
        if parse_rotation(self, class)? {
            return Ok(true);
        }

        Ok(false)
    }
}

fn parse_translate(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    // Computed style support (check before parse_neg)
    match class {
        "translate-x" => {
            insert_computed_style!(ctx, transform, TranslateX, "translation", 1);
        }
        "translate-y" => {
            insert_computed_style!(ctx, transform, TranslateY, "translation", 1);
        }
        "translate" => {
            insert_computed_style!(
                ctx,
                transform,
                [
                    (TranslateX, "translation", 0),
                    (TranslateY, "translation", 0)
                ]
            );
        }
        _ => {}
    }

    let (neg, class) = parse_neg(class);

    // translate-x-<value>
    if class.starts_with("translate-x-") {
        let val = parse_translate_val(&class["translate-x-".len()..], neg)?;
        deny_computed_style!(ctx);

        let val_tokens = val.to_token_stream();
        let full_val = quote::quote! { bevy::ui::Val2 { x: #val_tokens, ..Default::default() } };
        insert_picking_style!(ctx, TranslateX, full_val.clone());

        insert_transform_val2!(ctx, "translation", val, 1, ["x"]);
    }

    // translate-y-<value>
    if class.starts_with("translate-y-") {
        let val = parse_translate_val(&class["translate-y-".len()..], neg)?;
        deny_computed_style!(ctx);

        let val_tokens = val.to_token_stream();
        let full_val = quote::quote! { bevy::ui::Val2 { y: #val_tokens, ..Default::default() } };
        insert_picking_style!(ctx, TranslateY, full_val.clone());

        insert_transform_val2!(ctx, "translation", val, 1, ["y"]);
    }

    // translate-<value> (both x and y)
    // But not translate-x- or translate-y- which are handled above
    if class.starts_with("translate-")
        && !class.starts_with("translate-x-")
        && !class.starts_with("translate-y-")
        && class.len() > "translate-".len()
    {
        let val = parse_translate_val(&class["translate-".len()..], neg)?;
        deny_computed_style!(ctx);

        let val_tokens = val.to_token_stream();
        // Insert picking styles if hover/focus is active
        if ctx.hover || ctx.focus {
            ctx.insert_picking_style(
                PickingStyleProp::TranslateX,
                quote::quote! { bevy::ui::Val2 { x: #val_tokens, ..Default::default() } },
            );
            ctx.insert_picking_style(
                PickingStyleProp::TranslateY,
                quote::quote! { bevy::ui::Val2 { y: #val_tokens, ..Default::default() } },
            );
            return Ok(true);
        }

        insert_transform_val2!(ctx, "translation", val, 0, ["x", "y"]);
    }

    Ok(false)
}

fn parse_translate_val(val_str: &str, neg: bool) -> Result<Val, ParseClassError> {
    Val::parse(
        val_str,
        ParseValSettings::default_allow()
            .allow_fraction(true)
            .allow_full(true)
            .allow_px(true),
    )
    .and_then(|val| val.eval_neg(neg))
    .ok_or(ParseClassError::Unknown)
}

fn parse_scale(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    // Computed style support (check before parse_neg)
    match class {
        "scale-x" => {
            insert_computed_style!(ctx, transform, ScaleX, "scale", 1);
        }
        "scale-y" => {
            insert_computed_style!(ctx, transform, ScaleY, "scale", 1);
        }
        "scale" => {
            insert_computed_style!(ctx, transform, [(ScaleX, "scale", 0), (ScaleY, "scale", 0)]);
        }
        _ => {}
    }

    let (neg, class) = parse_neg(class);

    // scale-x-<value>
    if class.starts_with("scale-x-") {
        let val = parse_scale_val(&class["scale-x-".len()..], neg)?;
        deny_computed_style!(ctx);

        let full_val = quote::quote! { bevy::math::Vec2 { x: #val, ..Default::default() } };
        insert_picking_style!(ctx, ScaleX, full_val.clone());

        insert_transform_vec2!(ctx, "scale", val, 1, ["x"]);
    }

    // scale-y-<value>
    if class.starts_with("scale-y-") {
        let val = parse_scale_val(&class["scale-y-".len()..], neg)?;
        deny_computed_style!(ctx);

        let full_val = quote::quote! { bevy::math::Vec2 { y: #val, ..Default::default() } };
        insert_picking_style!(ctx, ScaleY, full_val.clone());

        insert_transform_vec2!(ctx, "scale", val, 1, ["y"]);
    }

    // scale-<value> (both x and y)
    // But not scale-x- or scale-y- which are handled above
    if class.starts_with("scale-")
        && !class.starts_with("scale-x-")
        && !class.starts_with("scale-y-")
        && class.len() > "scale-".len()
    {
        let val = parse_scale_val(&class["scale-".len()..], neg)?;
        deny_computed_style!(ctx);

        let full_val_x = quote::quote! { bevy::math::Vec2 { x: #val, ..Default::default() } };
        let full_val_y = quote::quote! { bevy::math::Vec2 { y: #val, ..Default::default() } };

        // Insert picking styles if hover/focus is active
        if ctx.hover || ctx.focus {
            ctx.insert_picking_style(PickingStyleProp::ScaleX, full_val_x);
            ctx.insert_picking_style(PickingStyleProp::ScaleY, full_val_y);
            return Ok(true);
        }

        insert_transform_vec2!(ctx, "scale", val, 0, ["x", "y"]);
    }

    Ok(false)
}

fn parse_scale_val(val_str: &str, neg: bool) -> Result<f32, ParseClassError> {
    // Parse percentage values like "scale-75" -> 0.75, "scale-100" -> 1.0, "scale-125" -> 1.25
    let val = if let Ok(num) = val_str.parse::<u32>() {
        num as f32 / 100.0
    } else if let Ok(num) = val_str.parse::<f32>() {
        num
    } else {
        return Err(ParseClassError::Unknown);
    };

    if neg {
        Ok(-val)
    } else {
        Ok(val)
    }
}

fn parse_rotation(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    // Computed style support
    if class == "rotate" {
        insert_computed_style!(ctx, transform, Rotation, "rotation", 0);
    }

    let (neg, class) = parse_neg(class);

    // rotate-<value> (in degrees)
    if class.starts_with("rotate-") {
        let val = parse_rotation_val(&class["rotate-".len()..], neg)?;
        deny_computed_style!(ctx);

        let val = quote::quote! { bevy::math::Rot2::degrees(#val) };
        insert_picking_style!(ctx, Rotation, val.clone());

        ctx.components
            .transform
            .insert("rotation", val, &ctx.class_type, 0);
        return Ok(true);
    }

    Ok(false)
}

fn parse_rotation_val(val_str: &str, neg: bool) -> Result<f32, ParseClassError> {
    // Parse degree values like "rotate-45" -> 45 degrees
    let degrees = if let Ok(num) = val_str.parse::<u32>() {
        num as f32
    } else if let Ok(num) = val_str.parse::<f32>() {
        num
    } else {
        return Err(ParseClassError::Unknown);
    };

    if neg {
        Ok(-degrees)
    } else {
        Ok(degrees)
    }
}

// Macro to insert Val2 properties with priorities (for translation)
macro_rules! insert_transform_val2 {
    ($ctx:ident, $prop:expr, $value:expr, $priority:literal, $fields:expr) => {{
        let s = $ctx
            .components
            .transform
            .props
            .entry($prop)
            .or_insert_with(|| {
                crate::utils::quote::StructVal::nested(
                    crate::utils::quote::Struct::<&'static str>::new(
                        quote::quote! {bevy::ui::Val2},
                    ),
                )
            })
            .as_nested_mut();

        for field in $fields {
            if let Some(prop) = s.props.get_mut(field) {
                prop.as_priotized_mut().insert(
                    $value.to_token_stream(),
                    &$ctx.class_type,
                    $priority,
                );
            } else {
                s.insert(field, $value.to_token_stream(), &$ctx.class_type, $priority);
            }
        }

        return Ok(true);
    }};
}

// Macro to insert Vec2 properties with priorities (for scale)
macro_rules! insert_transform_vec2 {
    ($ctx:ident, $prop:expr, $value:expr, $priority:literal, $fields:expr) => {{
        let val_f32 = $value; // Capture the f32 value
        let val_tokens = quote::quote! { #val_f32 }; // Convert to TokenStream

        let s = $ctx
            .components
            .transform
            .props
            .entry($prop)
            .or_insert_with(|| {
                crate::utils::quote::StructVal::nested(
                    crate::utils::quote::Struct::<&'static str>::new(
                        quote::quote! {bevy::math::Vec2},
                    ),
                )
            })
            .as_nested_mut();

        for field in $fields {
            if let Some(prop) = s.props.get_mut(field) {
                prop.as_priotized_mut()
                    .insert(val_tokens.clone(), &$ctx.class_type, $priority);
            } else {
                s.insert(field, val_tokens.clone(), &$ctx.class_type, $priority);
            }
        }

        return Ok(true);
    }};
}

pub(crate) use {insert_transform_val2, insert_transform_vec2};
