use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_grid_auto_flow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let grid_auto_flow = match class {
        "grid-flow-row" => quote! { bevy::ui::GridAutoFlow::Row },
        "grid-flow-col" => quote! { bevy::ui::GridAutoFlow::Column },
        "grid-flow-row-dense" => quote! { bevy::ui::GridAutoFlow::RowDense },
        "grid-flow-col-dense" => quote! { bevy::ui::GridAutoFlow::ColumnDense },
        _ => return Ok(false),
    };

    ctx.insert_node_prop(NodeProp::GridAutoFlow, grid_auto_flow);
    Ok(true)
}
