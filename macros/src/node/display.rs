use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_display(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let display = match class {
        "flex" => quote! { bevy::ui::Display::Flex },
        "grid" => quote! { bevy::ui::Display::Grid },
        "block" => quote! { bevy::ui::Display::Block },
        "hidden" => quote! { bevy::ui::Display::None },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::Display, display);
    Ok(true)
}
