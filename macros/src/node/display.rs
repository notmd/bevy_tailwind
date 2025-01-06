use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_display(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "flex" => {
            ctx.insert_node_prop_simple(NodeProp::Display, quote! {
                bevy::ui::Display::Flex
            });
            Ok(true)
        }
        "grid" => {
            ctx.insert_node_prop_simple(NodeProp::Display, quote! {
                bevy::ui::Display::Grid
            });
            Ok(true)
        }
        "block" => {
            ctx.insert_node_prop_simple(NodeProp::Display, quote! {
                bevy::ui::Display::Block
            });
            Ok(true)
        }
        "hidden" => {
            ctx.insert_node_prop_simple(NodeProp::Display, quote! {
                bevy::ui::Display::None
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
