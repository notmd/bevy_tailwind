use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_place_self(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (align_self, justity_self) = match class {
        "place-self-auto" => (
            quote! { bevy::ui::AlignSelf::Auto },
            quote! { bevy::ui::JustifySelf::Auto },
        ),
        "place-self-start" => (
            quote! { bevy::ui::AlignSelf::Start },
            quote! { bevy::ui::JustifySelf::Start},
        ),
        "place-self-end" => (
            quote! { bevy::ui::AlignSelf::End },
            quote! { bevy::ui::JustifySelf::End },
        ),
        "place-self-center" => (
            quote! { bevy::ui::AlignSelf::Center },
            quote! { bevy::ui::JustifySelf::Center },
        ),
        "place-self-stretch" => (
            quote! { bevy::ui::AlignSelf::Stretch },
            quote! { bevy::ui::JustifySelf::Stretch },
        ),
        _ => return Ok(false),
    };

    if !ctx.components.node.contains_key(&NodeProp::AlignSelf) {
        ctx.insert_node_prop_simple(NodeProp::AlignSelf, align_self);
    }

    if !ctx.components.node.contains_key(&NodeProp::JustifySelf) {
        ctx.insert_node_prop_simple(NodeProp::JustifySelf, justity_self);
    }

    Ok(true)
}
