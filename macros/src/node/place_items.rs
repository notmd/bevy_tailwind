use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_place_items(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (align_items, justify_items) = match class {
        "place-items-start" => (
            quote! { bevy::ui::AlignItems::Start },
            quote! { bevy::ui::JustifyItems::Start },
        ),
        "place-items-end" => (
            quote! { bevy::ui::AlignItems::End },
            quote! { bevy::ui::JustifyItems::End },
        ),
        "place-items-center" => (
            quote! { bevy::ui::AlignItems::Center },
            quote! { bevy::ui::JustifyItems::Center },
        ),
        "place-items-baseline" => (
            quote! { bevy::ui::AlignItems::Baseline },
            quote! { bevy::ui::JustifyItems::Baseline },
        ),
        "place-items-stretch" => (
            quote! { bevy::ui::AlignItems::Stretch },
            quote! { bevy::ui::JustifyItems::Stretch },
        ),

        _ => return Ok(false),
    };

    if !ctx.components.node.contains_key(&NodeProp::AlignItems) {
        ctx.insert_node_prop_simple(NodeProp::AlignItems, align_items);
    }

    if !ctx.components.node.contains_key(&NodeProp::JustifyItems) {
        ctx.insert_node_prop_simple(NodeProp::JustifyItems, justify_items);
    }

    Ok(true)
}
