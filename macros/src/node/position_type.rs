use super::NodeProp;
use crate::{picking::insert_picking_style, ParseCtx, ParseResult};
use quote::quote;

pub fn parse_position_type(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let position_type = match class {
        "relative" => quote! { bevy::ui::PositionType::Relative },
        "absolute" => quote! { bevy::ui::PositionType::Absolute },
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, Position, position_type.clone());
    ctx.insert_node_prop(NodeProp::PositionType, position_type);
    Ok(true)
}
