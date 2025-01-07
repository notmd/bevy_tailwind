use crate::{ParseCtx, ParseResult, utils::StructPropValue};
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
    pub fn parse_text_class(&mut self, class: &str) -> ParseResult {
        parse_class!(
            parse_font_size(self, class),
            parse_font_smoothing(self, class)
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

pub fn parse_font_smoothing(ctx: &mut ParseCtx, class: &str) -> ParseResult {
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
