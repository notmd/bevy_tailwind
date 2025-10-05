use quote::quote;

use crate::{
    ParseCtx, ParseResult,
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
};

use super::NodeProp;

pub fn parse_flex_direction(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "flex-dir" {
        insert_computed_style!(ctx, node, FlexDirection, NodeProp::FlexDirection, 0);
    }

    let flex_direction = match class {
        "flex-row" => quote! { bevy::ui::FlexDirection::Row },
        "flex-row-reverse" => quote! { bevy::ui::FlexDirection::RowReverse },
        "flex-col" => quote! { bevy::ui::FlexDirection::Column },
        "flex-col-reverse" => quote! { bevy::ui::FlexDirection::ColumnReverse },
        _ => return Ok(false),
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, FlexDirection, flex_direction);
    ctx.insert_node_prop(NodeProp::FlexDirection, flex_direction);

    Ok(true)
}
