use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_wrap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let flex_wrap = match class {
        "flex-wrap" => quote! { bevy::ui::FlexWrap::Wrap },
        "flex-wrap-reverse" => quote! { bevy::ui::FlexWrap::WrapReverse },
        "flex-nowrap" => quote! { bevy::ui::FlexWrap::NoWrap },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::FlexWrap, flex_wrap);

    Ok(true)
}
