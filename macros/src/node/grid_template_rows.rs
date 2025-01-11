use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_grid_template_rows(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("grid-rows-") {
        return Ok(false);
    }

    let suffix = &class["grid-rows-".len()..];

    match suffix {
        "none" => {
            ctx.insert_node_prop(NodeProp::GridTemplateRows, quote! {
                Default::default()
            });

            Ok(true)
        }
        _ => {
            let Ok(count) = suffix.parse::<u16>() else {
                return Ok(false);
            };

            ctx.insert_node_prop(NodeProp::GridTemplateRows, quote! {
               bevy::ui::RepeatedGridTrack::minmax(
                    #count,
                    bevy::ui::MinTrackSizingFunction::Px(0.),
                    bevy::ui::MaxTrackSizingFunction::Fraction(1.)
               )
            });

            Ok(true)
        }
    }
}
