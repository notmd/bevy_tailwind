use crate::{
    ParseCtx, ParseResult,
    utils::{StructPropValue, color::Color, val::Val},
};

impl ParseCtx {
    pub fn parse_outline(&mut self, class: &str) -> ParseResult {
        if !class.starts_with("outline-") {
            return Ok(false);
        }

        let class = &class["outline-".len()..];

        if let Ok(width) = class.parse::<u32>() {
            self.components.outline.insert(
                "width",
                StructPropValue::simple(self.class_type, Val::Px(width as f32)),
            );
            return Ok(true);
        }

        if class.starts_with("offset-") {
            let class = &class["offset-".len()..];
            if let Ok(offset) = class.parse::<u32>() {
                self.components.outline.insert(
                    "offset",
                    StructPropValue::simple(self.class_type, Val::Px(offset as f32)),
                );
                return Ok(true);
            }
        }

        if let Some(color) = Color::parse(class) {
            self.components
                .outline
                .insert("color", StructPropValue::simple(self.class_type, color));
            return Ok(true);
        }

        Ok(false)
    }
}
