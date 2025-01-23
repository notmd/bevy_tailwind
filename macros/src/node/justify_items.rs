use super::NodeProp;
use crate::{picking::insert_picking_style, ParseCtx, ParseResult};
use quote::quote;

pub fn parse_justify_items(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let justify_items = match class {
        "justify-items-start" => quote! { bevy::ui::JustifyItems::Start },
        "justify-items-end" => quote! { bevy::ui::JustifyItems::End },
        "justify-items-center" => quote! { bevy::ui::JustifyItems::Center },
        "justify-items-stretch" => quote! { bevy::ui::JustifyItems::Stretch },
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, JustifyItems, justify_items);
    ctx.insert_node_prop_priority(NodeProp::JustifyItems, justify_items, 1);
    Ok(true)
}
