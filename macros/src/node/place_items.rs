use crate::{picking::deny_picking_style, utils::deny_computed_style, ParseCtx, ParseResult};
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

    deny_picking_style!(ctx);
    deny_computed_style!(ctx);
    ctx.insert_node_prop_priority(NodeProp::AlignItems, align_items, 0);
    ctx.insert_node_prop_priority(NodeProp::JustifyItems, justify_items, 0);

    Ok(true)
}
