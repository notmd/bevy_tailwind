use quote::quote;

use crate::{ParseClassError, ParseCtx, ParseResult, utils::parse_neg};

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

        self.z_index = Some(quote! {
           #z_index
        });

        return Ok(true);
    }

    pub fn get_z_index(&self) -> Option<proc_macro2::TokenStream> {
        self.z_index.clone().map(|prop| {
            self.quote_tuple_component(
                quote! {
                    bevy::ui::ZIndex
                },
                vec![prop],
            )
        })
    }
}
