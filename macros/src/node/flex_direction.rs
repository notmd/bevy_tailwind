use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_direction(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let token = match class {
        "flex-row" => {
            quote! {
                bevy::ui::FlexDirection::Row
            }
        }
        "flex-row-reverse" => {
            quote! {
                bevy::ui::FlexDirection::RowReverse
            }
        }
        "flex-col" => {
            quote! {
                bevy::ui::FlexDirection::Column
            }
        }
        "flex-col-reverse" => {
            quote! {
                bevy::ui::FlexDirection::ColumnReverse
            }
        }
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::FlexDirection, token);

    Ok(true)
}
