use quote::ToTokens;

use crate::{
    picking::{insert_picking_style, PickingStyleProp},
    utils::color::Color,
    ClassType, ParseCtx, ParseResult,
};

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
    pub fn parse_background(&mut self, class: &str) -> ParseResult {
        parse_class!(parse_background_color(self, class));
        return Ok(false);
    }
}

fn parse_background_color(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match &ctx.class_type {
        ClassType::Computed(expr) if class == "bg" => {
            insert_picking_style!(ctx, BackgroundColor, expr.to_token_stream());
            ctx.components
                .background_color
                .insert("0", expr.to_token_stream(), &ctx.class_type, 0);
            return Ok(true);
        }
        _ => {}
    }

    if !class.starts_with("bg-") {
        return Ok(false);
    }

    let Some(color) = Color::parse(&class["bg-".len()..]) else {
        return Ok(false);
    };

    insert_picking_style!(ctx, BackgroundColor, color);

    ctx.components
        .background_color
        .insert("0", color, &ctx.class_type, 0);

    Ok(true)
}
