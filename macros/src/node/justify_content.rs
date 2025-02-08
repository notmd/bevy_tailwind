use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
    ParseCtx, ParseResult,
};
use quote::quote;

use super::NodeProp;

pub fn parse_justify_content(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "justity" {
        insert_computed_style!(ctx, node, JustifyContent, NodeProp::JustifyContent, 1);
    }

    let justify_content = match class {
        "justify-normal" => quote! { bevy::ui::JustifyContent::Default },
        "justify-start" => quote! { bevy::ui::JustifyContent::FlexStart },
        "justify-end" => quote! { bevy::ui::JustifyContent::FlexEnd },
        "justify-center" => quote! { bevy::ui::JustifyContent::Center },
        "justify-between" => quote! { bevy::ui::JustifyContent::SpaceBetween },
        "justify-around" => quote! { bevy::ui::JustifyContent::SpaceAround },
        "justify-evenly" => quote! { bevy::ui::JustifyContent::SpaceEvenly },
        "justify-stretch" => quote! { bevy::ui::JustifyContent::Stretch },
        _ => return Ok(false),
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, JustifyContent, justify_content);
    ctx.insert_node_prop_priority(NodeProp::JustifyContent, justify_content, 1);
    Ok(true)
}
