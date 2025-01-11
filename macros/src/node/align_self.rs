use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_align_self(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let align_self = match class {
        "self-auto" => quote! { bevy::ui::AlignSelf::Auto },
        "self-start" => quote! { bevy::ui::AlignSelf::FlexStart },
        "self-end" => quote! { bevy::ui::AlignSelf::FlexEnd },
        "self-center" => quote! { bevy::ui::AlignSelf::Center },
        "self-stretch" => quote! { bevy::ui::AlignSelf::Stretch },
        "self-baseline" => quote! { bevy::ui::AlignSelf::Baseline },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_priority(NodeProp::AlignSelf, align_self, 1);
    Ok(true)
}
