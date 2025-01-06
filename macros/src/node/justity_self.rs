use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_justify_self(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "justify-self-auto" => {
            ctx.insert_node_prop_simple(NodeProp::JustifySelf, quote! {
                bevy::ui::JustifySelf::Auto
            });

            Ok(true)
        }
        "justify-self-start" => {
            ctx.insert_node_prop_simple(NodeProp::JustifySelf, quote! {
                bevy::ui::JustifySelf::Start
            });

            Ok(true)
        }
        "justify-self-end" => {
            ctx.insert_node_prop_simple(NodeProp::JustifySelf, quote! {
                bevy::ui::JustifySelf::End
            });

            Ok(true)
        }
        "justify-self-center" => {
            ctx.insert_node_prop_simple(NodeProp::JustifySelf, quote! {
                bevy::ui::JustifySelf::Center
            });

            Ok(true)
        }
        "justify-self-stretch" => {
            ctx.insert_node_prop_simple(NodeProp::JustifySelf, quote! {
                bevy::ui::JustifySelf::Stretch
            });

            Ok(true)
        }
        _ => Ok(false),
    }
}
