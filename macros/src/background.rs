use crate::{
    picking::insert_picking_style,
    utils::{color::Color, deny_computed_style, insert_computed_style},
    ParseCtx, ParseResult,
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
        Ok(false)
    }
}

fn parse_background_color(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "bg" {
        insert_computed_style!(ctx, background_color, BackgroundColor, "0", 0);
    }

    if !class.starts_with("bg-") {
        return Ok(false);
    }

    let Some(color) = Color::parse(&class["bg-".len()..]) else {
        return Ok(false);
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, BackgroundColor, color);

    ctx.components
        .background_color
        .insert("0", color, &ctx.class_type, 0);

    Ok(true)
}
