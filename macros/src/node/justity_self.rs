use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_justify_self(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let justify_self = match class {
        "justify-self-auto" => quote! { bevy::ui::JustifySelf::Auto },
        "justify-self-start" => quote! { bevy::ui::JustifySelf::Start },
        "justify-self-end" => quote! { bevy::ui::JustifySelf::End },
        "justify-self-center" => quote! { bevy::ui::JustifySelf::Center },
        "justify-self-stretch" => quote! { bevy::ui::JustifySelf::Stretch },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::JustifySelf, justify_self);
    Ok(true)
}
