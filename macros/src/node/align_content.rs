use crate::{
    ParseCtx, ParseResult,
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
};

use super::NodeProp;
use quote::quote;

pub fn parse_align_content(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "content" {
        insert_computed_style!(ctx, node, AlignContent, NodeProp::AlignContent, 1);
    }

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

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, AlignContent, align_content);
    ctx.insert_node_prop_priority(NodeProp::AlignContent, align_content, 1);
    Ok(true)
}
