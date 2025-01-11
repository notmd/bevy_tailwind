use crate::{
    utils::{color::Color, val::Val},
    ParseCtx, ParseResult,
};

impl ParseCtx {
    pub fn parse_outline(&mut self, class: &str) -> ParseResult {
        if !class.starts_with("outline-") {
            return Ok(false);
        }

        let class = &class["outline-".len()..];

        if let Ok(width) = class.parse::<u32>() {
            self.components
                .outline
                .insert("width", Val::Px(width as f32), self.class_type, 0);
            return Ok(true);
        }

        if class.starts_with("offset-") {
            let class = &class["offset-".len()..];
            if let Ok(offset) = class.parse::<u32>() {
                self.components.outline.insert(
                    "offset",
                    Val::Px(offset as f32),
                    self.class_type,
                    0,
                );
                return Ok(true);
            }
        }

        if let Some(color) = Color::parse(class) {
            self.components
                .outline
                .insert("color", color, self.class_type, 0);
            return Ok(true);
        }

        Ok(false)
    }
}
