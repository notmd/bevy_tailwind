use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_justify_content(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "justify-normal" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::Default
            });

            Ok(true)
        }
        "justify-start" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::Start
            });

            Ok(true)
        }
        "justify-end" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::End
            });

            Ok(true)
        }
        "justify-center" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::Center
            });

            Ok(true)
        }
        "justify-between" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::SpaceBetween
            });

            Ok(true)
        }
        "justify-around" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::SpaceAround
            });

            Ok(true)
        }
        "justify-evenly" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::SpaceEvenly
            });

            Ok(true)
        }
        "justify-stretch" => {
            ctx.insert_node_prop_simple(NodeProp::JustifyContent, quote! {
                bevy::ui::JustifyContent::Stretch
            });

            Ok(true)
        }
        _ => Ok(false),
    }
}
