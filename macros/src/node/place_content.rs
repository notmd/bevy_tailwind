use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_place_content(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let (align_content, justify_content) = match class {
        "place-content-center" => (
            quote! { bevy::ui::AlignContent::Center },
            quote! { bevy::ui::JustifyContent::Center },
        ),
        "place-content-start" => (
            quote! { bevy::ui::AlignContent::Start },
            quote! { bevy::ui::JustifyContent::Start },
        ),
        "place-content-end" => (
            quote! { bevy::ui::AlignContent::End },
            quote! { bevy::ui::JustifyContent::End },
        ),
        "place-content-between" => (
            quote! { bevy::ui::AlignContent::SpaceBetween },
            quote! { bevy::ui::JustifyContent::SpaceBetween },
        ),
        "place-content-around" => (
            quote! { bevy::ui::AlignContent::SpaceAround },
            quote! { bevy::ui::JustifyContent::SpaceAround },
        ),
        "place-content-evenly" => (
            quote! { bevy::ui::AlignContent::SpaceEvenly },
            quote! { bevy::ui::JustifyContent::SpaceEvenly },
        ),
        "place-content-baseline" => (
            quote! { bevy::ui::AlignContent::Stretch },
            quote! { bevy::ui::JustifyContent::Stretch },
        ),
        "place-content-stretch" => (
            quote! { bevy::ui::AlignContent::Stretch },
            quote! { bevy::ui::JustifyContent::Stretch },
        ),
        _ => return Ok(false),
    };

    ctx.insert_node_prop_priority(NodeProp::AlignContent, align_content, 0);

    ctx.insert_node_prop_priority(NodeProp::JustifyContent, justify_content, 0);

    Ok(true)
}
