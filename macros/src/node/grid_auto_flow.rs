use crate::{picking::insert_picking_style, utils::insert_computed_style, ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_grid_auto_flow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "grid-flow" {
        insert_computed_style!(ctx, node, GridAutoFlow, NodeProp::GridAutoFlow, 0);
    }

    let grid_auto_flow = match class {
        "grid-flow-row" => quote! { bevy::ui::GridAutoFlow::Row },
        "grid-flow-col" => quote! { bevy::ui::GridAutoFlow::Column },
        "grid-flow-row-dense" => quote! { bevy::ui::GridAutoFlow::RowDense },
        "grid-flow-col-dense" => quote! { bevy::ui::GridAutoFlow::ColumnDense },
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, GridAutoFlow, grid_auto_flow);
    ctx.insert_node_prop(NodeProp::GridAutoFlow, grid_auto_flow);
    Ok(true)
}
