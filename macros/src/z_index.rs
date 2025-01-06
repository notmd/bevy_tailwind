use quote::quote;

use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::{StructPropValue, parse_neg},
};

impl ParseCtx {
    pub fn parse_z_index(&mut self, class: &str) -> ParseResult {
        let (neg, class) = parse_neg(class);
        let Some(z_index) = class.strip_prefix("z-") else {
            return Ok(false);
        };

        let z_index = z_index
            .parse::<u32>()
            .map_err(|_| ParseClassError::Unsupported)?;
        let z_index = if neg {
            -(z_index as i32)
        } else {
            z_index as i32
        };

        if neg && z_index == 0 {
            return Err(ParseClassError::Unsupported);
        }

        self.components.z_index.insert(
            "0",
            StructPropValue::simple(self.class_type, quote! {#z_index}),
        );
        return Ok(true);
    }
}
