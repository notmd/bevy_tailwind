use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_grid_auto_flow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("grid-flow-") {
        return Ok(false);
    }

    let suffix = &class["grid-flow-".len()..];

    match suffix {
        "row" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoFlow, quote! {
                bevy::ui::GridAutoFlow::Row
            });

            Ok(true)
        }
        "col" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoFlow, quote! {
                bevy::ui::GridAutoFlow::Column
            });

            Ok(true)
        }
        "row-dense" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoFlow, quote! {
                bevy::ui::GridAutoFlow::RowDense
            });

            Ok(true)
        }
        "col-dense" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoFlow, quote! {
                bevy::ui::GridAutoFlow::ColumnDense
            });

            Ok(true)
        }

        _ => Ok(false),
    }
}
