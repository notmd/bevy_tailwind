use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_align_content(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let align_content = match class {
        "content-normal" => quote! { bevy::ui::AlignContent::Default },
        "content-center" => quote! { bevy::ui::AlignContent::Center },
        "content-start" => quote! { bevy::ui::AlignContent::FlexStart },
        "content-end" => quote! { bevy::ui::AlignContent::FlexEnd },
        "content-between" => quote! { bevy::ui::AlignContent::SpaceBetween },
        "content-around" => quote! { bevy::ui::AlignContent::SpaceAround },
        "content-evenly" => quote! { bevy::ui::AlignContent::SpaceEvenly },
        "content-stretch" => quote! { bevy::ui::AlignContent::Stretch },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_priority(NodeProp::AlignContent, align_content, 1);
    Ok(true)
}
