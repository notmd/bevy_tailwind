use crate::{ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_grid_auto_rows(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "auto-rows-auto" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoRows, quote! {
                bevy::ui::GridTrack::auto()
            });

            Ok(true)
        }
        "auto-rows-min" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoRows, quote! {
                bevy::ui::GridTrack::min_content()
            });

            Ok(true)
        }
        "auto-rows-max" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoRows, quote! {
                bevy::ui::GridTrack::max_content()
            });

            Ok(true)
        }
        "auto-rows-fr" => {
            ctx.insert_node_prop_simple(NodeProp::GridAutoRows, quote! {
                bevy::ui::GridTrack::flex(1.)
            });

            Ok(true)
        }
        _ => Ok(false),
    }
}
