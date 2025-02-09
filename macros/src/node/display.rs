use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
    ParseCtx, ParseResult,
};
use quote::quote;

use super::NodeProp;

pub fn parse_display(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "d" {
        insert_computed_style!(ctx, node, Display, NodeProp::Display, 0);
    }

    let display = match class {
        "flex" => quote! { bevy::ui::Display::Flex },
        "grid" => quote! { bevy::ui::Display::Grid },
        "block" => quote! { bevy::ui::Display::Block },
        "hidden" => quote! { bevy::ui::Display::None },
        _ => return Ok(false),
    };
    deny_computed_style!(ctx);
    insert_picking_style!(ctx, Display, display);
    ctx.insert_node_prop(NodeProp::Display, display);

    Ok(true)
}
