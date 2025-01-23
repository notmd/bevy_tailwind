use crate::{picking::insert_picking_style, ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_grid_template_rows(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("grid-rows-") {
        return Ok(false);
    }

    let suffix = &class["grid-rows-".len()..];

    match suffix {
        "none" => {
            let value = quote! {
                Default::default()
            };
            insert_picking_style!(ctx, GridTemplateRows, value);
            ctx.insert_node_prop(NodeProp::GridTemplateRows, value);

            Ok(true)
        }
        _ => {
            let Ok(count) = suffix.parse::<u16>() else {
                return Ok(false);
            };
            let value = quote! {
               bevy::ui::RepeatedGridTrack::minmax(
                    #count,
                    bevy::ui::MinTrackSizingFunction::Px(0.),
                    bevy::ui::MaxTrackSizingFunction::Fraction(1.)
               )
            };
            insert_picking_style!(ctx, GridTemplateRows, value);
            ctx.insert_node_prop(NodeProp::GridTemplateRows, value);

            Ok(true)
        }
    }
}
