use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_grid_auto_columns(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let prop = match class {
        "auto-cols-auto" => quote! { bevy::ui::GridTrack::auto() },
        "auto-cols-min" => quote! { bevy::ui::GridTrack::min_content() },
        "auto-cols-max" => quote! { bevy::ui::GridTrack::max_content() },
        "auto-cols-fr" => quote! { bevy::ui::GridTrack::flex(1.) },
        _ => return Ok(false),
    };

    ctx.insert_node_prop_simple(NodeProp::GridAutoColumns, prop);
    Ok(true)
}
