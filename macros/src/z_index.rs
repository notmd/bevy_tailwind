use quote::quote;

use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style, parse_neg},
    ParseClassError, ParseCtx, ParseResult,
};

impl ParseCtx {
    pub fn parse_z_index(&mut self, class: &str) -> ParseResult {
        if class == "z" {
            insert_computed_style!(self, z_index, ZIndex, "0", 1);
        }

        let (neg, class) = parse_neg(class);
        let Some(z_index) = class.strip_prefix("z-") else {
            return Ok(false);
        };

        let z_index = z_index
            .parse::<u32>()
            .map_err(|_| ParseClassError::Unknown)?;
        let z_index = if neg {
            -(z_index as i32)
        } else {
            z_index as i32
        };

        if neg && z_index == 0 {
            return Err(ParseClassError::Unknown);
        }

        deny_computed_style!(self);
        let z_index = quote! {#z_index};
        insert_picking_style!(self, ZIndex, z_index);
        self.components
            .z_index
            .insert("0", z_index, &self.class_type, 0);

        Ok(true)
    }
}
