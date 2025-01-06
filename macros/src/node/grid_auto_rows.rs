use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_grid_auto_rows(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let grid_auto_rows = match class {
        "auto-rows-auto" => quote! { bevy::ui::GridTrack::auto() },
        "auto-rows-min" => quote! { bevy::ui::GridTrack::min_content() },
        "auto-rows-max" => quote! { bevy::ui::GridTrack::max_content() },
        "auto-rows-fr" => quote! { bevy::ui::GridTrack::flex(1.) },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::GridAutoRows, grid_auto_rows);
    Ok(true)
}
