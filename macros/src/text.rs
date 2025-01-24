use crate::{
    picking::insert_picking_style,
    utils::{color::Color, val::parse_px},
    ParseCtx, ParseResult,
};
use quote::quote;

macro_rules! parse_class {
    ($($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    return Ok(true)
                }
                Err(e) => {
                  return Err(e);
                }
                _ => {}
            }
        )*
    };
}

impl ParseCtx {
    pub fn parse_text(&mut self, class: &str) -> ParseResult {
        parse_class!(
            parse_font_size(self, class),
            parse_font_smoothing(self, class),
            parse_text_align(self, class),
            parse_line_break(self, class),
            parse_text_color(self, class)
        );
        Ok(false)
    }
}

fn parse_font_size(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("text-") {
        return Ok(false);
    }

    let class = &class["text-".len()..];

    let font_size: f32 = match class {
        "xs" => 12.,
        "sm" => 14.,
        "base" => 16.,
        "lg" => 18.,
        "xl" => 20.,
        "2xl" => 24.,
        "3xl" => 30.,
        "4xl" => 36.,
        "5xl" => 48.,
        "6xl" => 64.,
        "7xl" => 80.,
        "8xl" => 96.,
        "9xl" => 128.,
        class if class.starts_with("[") && class.ends_with("]") => {
            let class = &class[1..class.len() - 1];
            let Some(px) = parse_px(class) else {
                return Ok(false);
            };
            px
        }
        _ => {
            return Ok(false);
        }
    };

    insert_picking_style!(ctx, FontSize, quote! { #font_size });

    ctx.components
        .text_font
        .insert("font_size", quote! {#font_size}, ctx.class_type, 0);

    Ok(true)
}

fn parse_font_smoothing(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class != "antialiased" {
        return Ok(false);
    }

    ctx.components.text_font.insert(
        "font_smoothing",
        quote! {
            bevy::text::FontSmoothing::AntiAliased
        },
        ctx.class_type,
        0,
    );

    return Ok(true);
}

fn parse_text_align(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("text-") {
        return Ok(false);
    }

    let class = &class["text-".len()..];

    let justify = match class {
        "left" => quote! {bevy::text::JustifyText::Left},
        "center" => quote! {bevy::text::JustifyText::Center},
        "right" => quote! {bevy::text::JustifyText::Right},
        "justify" => quote! {bevy::text::JustifyText::Justified},
        _ => {
            return Ok(false);
        }
    };

    insert_picking_style!(ctx, TextJustify, justify);

    ctx.components
        .text_layout
        .insert("justify", justify, ctx.class_type, 0);

    Ok(true)
}

fn parse_line_break(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let line_break = match class {
        "break-words" => quote! {bevy::text::LineBreak::WordBoundary},
        "break-all" => quote! {bevy::text::LineBreak::AnyCharacter},
        _ => {
            return Ok(false);
        }
    };

    insert_picking_style!(ctx, TextLinebreak, line_break);

    ctx.components
        .text_layout
        .insert("linebreak", line_break, ctx.class_type, 0);

    Ok(true)
}

fn parse_text_color(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("text-") {
        return Ok(false);
    }

    let Some(color) = Color::parse(&class["text-".len()..]) else {
        return Ok(false);
    };

    insert_picking_style!(ctx, TextColor, color);

    ctx.components
        .text_color
        .insert("0", color, ctx.class_type, 0);

    Ok(true)
}
