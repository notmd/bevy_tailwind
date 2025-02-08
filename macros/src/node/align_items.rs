use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
    ParseCtx, ParseResult,
};
use quote::quote;

use super::NodeProp;

pub fn parse_align_items(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "items" {
        insert_computed_style!(ctx, node, AlignItems, NodeProp::AlignItems, 1);
    }

    let align_items = match class {
        "items-start" => quote! { bevy::ui::AlignItems::FlexStart },
        "items-end" => quote! { bevy::ui::AlignItems::FlexEnd },
        "items-center" => quote! { bevy::ui::AlignItems::Center },
        "items-baseline" => quote! { bevy::ui::AlignItems::Baseline },
        "items-stretch" => quote! { bevy::ui::AlignItems::Stretch },
        _ => return Ok(false),
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, AlignItems, align_items);
    ctx.insert_node_prop_priority(NodeProp::AlignItems, align_items, 1);
    Ok(true)
}
