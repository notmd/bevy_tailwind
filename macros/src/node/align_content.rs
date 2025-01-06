use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_align_content(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "content-normal" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::Default
            });

            Ok(true)
        }
        "content-center" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::Center
            });

            Ok(true)
        }
        "content-start" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::FlexStart
            });

            Ok(true)
        }
        "content-end" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::FlexEnd
            });

            Ok(true)
        }

        "content-between" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::SpaceBetween
            });

            Ok(true)
        }
        "content-around" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::SpaceAround
            });

            Ok(true)
        }
        "content-evenly" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::SpaceEvenly
            });

            Ok(true)
        }
        "content-stretch" => {
            ctx.insert_node_prop_simple(NodeProp::AlignContent, quote! {
                bevy::ui::AlignContent::Stretch
            });

            Ok(true)
        }
        _ => Ok(false),
    }
}
