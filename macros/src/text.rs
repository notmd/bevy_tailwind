use crate::{
    ParseCtx, ParseResult,
    utils::{StructPropValue, color::Color},
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
        _ => {
            return Ok(false);
        }
    };

    ctx.components.text_font.insert(
        "font_size",
        StructPropValue::simple(ctx.class_type, quote! {#font_size}),
    );

    Ok(true)
}

fn parse_font_smoothing(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class != "antialiased" {
        return Ok(false);
    }

    ctx.components.text_font.insert(
        "font_smoothing",
        StructPropValue::simple(ctx.class_type, quote! {
            bevy::text::FontSmoothing::AntiAliased
        }),
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

    ctx.components
        .text_layouut
        .insert("justify", StructPropValue::simple(ctx.class_type, justify));

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

    ctx.components.text_layouut.insert(
        "linebreak",
        StructPropValue::simple(ctx.class_type, line_break),
    );

    Ok(true)
}

fn parse_text_color(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("text-") {
        return Ok(false);
    }

    let Some(color) = Color::parse(&class["text-".len()..]) else {
        return Ok(false);
    };

    ctx.components
        .text_color
        .insert("0", StructPropValue::simple(ctx.class_type, color));

    Ok(true)
}
