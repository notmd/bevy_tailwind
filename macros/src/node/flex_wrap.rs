use quote::quote;

use crate::{picking::insert_picking_style, ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_wrap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let flex_wrap = match class {
        "flex-wrap" => quote! { bevy::ui::FlexWrap::Wrap },
        "flex-wrap-reverse" => quote! { bevy::ui::FlexWrap::WrapReverse },
        "flex-nowrap" => quote! { bevy::ui::FlexWrap::NoWrap },
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, FlexWrap, flex_wrap);
    ctx.insert_node_prop(NodeProp::FlexWrap, flex_wrap);

    Ok(true)
}
