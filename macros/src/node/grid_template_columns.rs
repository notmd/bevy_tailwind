use quote::quote;

use crate::{picking::insert_picking_style, ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_grid_template_columns(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("grid-cols-") {
        return Ok(false);
    }

    let suffix = &class["grid-cols-".len()..];

    match suffix {
        "none" => {
            let value = quote! {
                Default::default()
            };
            insert_picking_style!(ctx, GridTemplateColumns, value);
            ctx.insert_node_prop(NodeProp::GridTemplateColumns, value);

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

            insert_picking_style!(ctx, GridTemplateColumns, value);
            ctx.insert_node_prop(NodeProp::GridTemplateColumns, value);

            Ok(true)
        }
    }
}
