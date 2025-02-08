use crate::{picking::insert_picking_style, utils::insert_computed_style, ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_grid_auto_columns(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "auto-cols" {
        insert_computed_style!(ctx, node, GridAutoColumns, NodeProp::GridAutoColumns, 0);
    }

    let prop = match class {
        "auto-cols-auto" => quote! { bevy::ui::GridTrack::auto() },
        "auto-cols-min" => quote! { bevy::ui::GridTrack::min_content() },
        "auto-cols-max" => quote! { bevy::ui::GridTrack::max_content() },
        "auto-cols-fr" => quote! { bevy::ui::GridTrack::flex(1.) },
        _ => return Ok(false),
    };

    insert_picking_style!(ctx, GridAutoColumns, prop);
    ctx.insert_node_prop(NodeProp::GridAutoColumns, prop);
    Ok(true)
}
