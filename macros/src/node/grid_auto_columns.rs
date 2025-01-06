use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_grid_auto_columns(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "auto-cols-auto" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoColumns, quote! {
                bevy::ui::GridTrack::auto()
            });

            Ok(true)
        }
        "auto-cols-min" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoColumns, quote! {
                bevy::ui::GridTrack::min_content()
            });

            Ok(true)
        }
        "auto-cols-max" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoColumns, quote! {
                bevy::ui::GridTrack::max_content()
            });

            Ok(true)
        }
        "auto-cols-fr" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoColumns, quote! {
                bevy::ui::GridTrack::flex(1.)
            });

            Ok(true)
        }
        _ => Ok(false),
    }
}
