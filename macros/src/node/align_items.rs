use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_align_items(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let align_items = match class {
        "items-start" => quote! { bevy::ui::AlignItems::FlexStart },
        "items-end" => quote! { bevy::ui::AlignItems::FlexEnd },
        "items-center" => quote! { bevy::ui::AlignItems::Center },
        "items-baseline" => quote! { bevy::ui::AlignItems::Baseline },
        "items-stretch" => quote! { bevy::ui::AlignItems::Stretch },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_priority(NodeProp::AlignItems, align_items, 1);
    Ok(true)
}
