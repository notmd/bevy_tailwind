use proc_macro2::TokenStream;

use crate::{ParseCtx, ParseResult};

impl ParseCtx {
    pub fn parse_background_color_class(&mut self, class: &str) -> ParseResult {
        return Ok(false);
    }

    pub fn get_background_color(&self) -> Option<TokenStream> {
        return None;
    }
}
