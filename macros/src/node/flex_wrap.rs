use quote::quote;

use crate::{
    ParseCtx, ParseResult,
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
};

use super::NodeProp;

pub fn parse_flex_wrap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "flex-wrap" {
        insert_computed_style!(ctx, node, FlexWrap, NodeProp::FlexWrap, 0);
    }

    let flex_wrap = match class {
        "flex-wrap" => quote! { bevy::ui::FlexWrap::Wrap },
        "flex-wrap-reverse" => quote! { bevy::ui::FlexWrap::WrapReverse },
        "flex-nowrap" => quote! { bevy::ui::FlexWrap::NoWrap },
        _ => return Ok(false),
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, FlexWrap, flex_wrap);
    ctx.insert_node_prop(NodeProp::FlexWrap, flex_wrap);

    Ok(true)
}
