use super::NodeProp;
use crate::{ParseCtx, ParseResult};
use quote::quote;

pub fn parse_justify_items(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "justify-items-start" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyItems, quote! {
                bevy::ui::JustifyItems::Start
            });

            Ok(true)
        }
        "justify-items-end" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyItems, quote! {
                bevy::ui::JustifyItems::End
            });

            Ok(true)
        }
        "justify-items-center" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyItems, quote! {
                bevy::ui::JustifyItems::Center
            });

            Ok(true)
        }
        "justify-items-stretch" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyItems, quote! {
                bevy::ui::JustifyItems::Stretch
            });

            Ok(true)
        }
        _ => Ok(false),
    }
}
