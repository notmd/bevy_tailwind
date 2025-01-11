use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_grid_template_columns(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("grid-cols-") {
        return Ok(false);
    }

    let suffix = &class["grid-cols-".len()..];

    match suffix {
        "none" => {
            ctx.insert_node_prop(NodeProp::GridTemplateColumns, quote! {
                Default::default()
            });

            Ok(true)
        }
        _ => {
            let Ok(count) = suffix.parse::<u16>() else {
                return Ok(false);
            };

            ctx.insert_node_prop(NodeProp::GridTemplateColumns, quote! {
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
