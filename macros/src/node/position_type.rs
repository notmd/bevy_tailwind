use super::NodeProp;
use crate::{
    ParseCtx, ParseResult,
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
};
use quote::quote;

pub fn parse_position_type(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "position" {
        insert_computed_style!(ctx, node, Position, NodeProp::PositionType, 0);
    }

    let position_type = match class {
        "relative" => quote! { bevy::ui::PositionType::Relative },
        "absolute" => quote! { bevy::ui::PositionType::Absolute },
        _ => return Ok(false),
    };
    deny_computed_style!(ctx);
    insert_picking_style!(ctx, Position, position_type.clone());
    ctx.insert_node_prop(NodeProp::PositionType, position_type);
    Ok(true)
}
