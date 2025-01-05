use super::NodeProp;
use crate::{ParseCtx, ParseResult};
use quote::quote;

pub fn parse_position_type(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "relative" => {
            ctx.insert_node_prop(NodeProp::PositionType, quote! {
                bevy::ui::PositionType::Relative
            });
            Ok(true)
        }
        "absolute" => {
            ctx.insert_node_prop(NodeProp::PositionType, quote! {
                bevy::ui::PositionType::Absolute
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
