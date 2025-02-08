use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
    ParseCtx, ParseResult,
};

use super::NodeProp;
use quote::quote;

pub fn parse_grid_auto_rows(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "auto-rows" {
        insert_computed_style!(ctx, node, GridAutoRows, NodeProp::GridAutoRows, 0);
    }

    let grid_auto_rows = match class {
        "auto-rows-auto" => quote! { bevy::ui::GridTrack::auto() },
        "auto-rows-min" => quote! { bevy::ui::GridTrack::min_content() },
        "auto-rows-max" => quote! { bevy::ui::GridTrack::max_content() },
        "auto-rows-fr" => quote! { bevy::ui::GridTrack::flex(1.) },
        _ => return Ok(false),
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, GridAutoRows, grid_auto_rows);
    ctx.insert_node_prop(NodeProp::GridAutoRows, grid_auto_rows);
    Ok(true)
}
