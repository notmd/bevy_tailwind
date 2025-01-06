use super::NodeProp;
use crate::{ParseCtx, ParseResult};
use quote::quote;

pub fn parse_position_type(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let position_type = match class {
        "relative" => quote! { bevy::ui::PositionType::Relative },
        "absolute" => quote! { bevy::ui::PositionType::Absolute },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::PositionType, position_type);
    Ok(true)
}
