use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_display(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "flex" => {
            ctx.node_props.insert(NodeProp::Display, quote! {
                bevy::ui::Display::Flex
            });
            Ok(true)
        }
        "grid" => {
            ctx.node_props.insert(NodeProp::Display, quote! {
                bevy::ui::Display::Grid
            });
            Ok(true)
        }
        "block" => {
            ctx.node_props.insert(NodeProp::Display, quote! {
                bevy::ui::Display::Block
            });
            Ok(true)
        }
        "hidden" => {
            ctx.node_props.insert(NodeProp::Display, quote! {
                bevy::ui::Display::None
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
