use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    picking::insert_picking_style,
    utils::{
        deny_computed_style, insert_computed_style,
        quote::ToTokenStream,
        val::{parse_percent, parse_px},
    },
    ParseCtx, ParseResult,
};

use super::NodeProp;

pub fn parse_grid_template_columns(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "grid-cols" {
        insert_computed_style!(
            ctx,
            node,
            GridTemplateColumns,
            NodeProp::GridTemplateColumns,
            0
        );
    }

    if !class.starts_with("grid-cols-") {
        return Ok(false);
    }

    let suffix = &class["grid-cols-".len()..];

    if let Some(val) = parse_arbitrary_grid_template(suffix) {
        deny_computed_style!(ctx);
        insert_picking_style!(ctx, GridTemplateColumns, val);
        ctx.insert_node_prop(NodeProp::GridTemplateColumns, val);

        return Ok(true);
    }

    match suffix {
        "none" => {
            let value = quote! {
                Default::default()
            };
            deny_computed_style!(ctx);
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

            deny_computed_style!(ctx);
            insert_picking_style!(ctx, GridTemplateColumns, value);
            ctx.insert_node_prop(NodeProp::GridTemplateColumns, value);

            Ok(true)
        }
    }
}

pub fn parse_grid_template_rows(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "grid-rows" {
        insert_computed_style!(ctx, node, GridTemplateRows, NodeProp::GridTemplateRows, 0);
    }

    if !class.starts_with("grid-rows-") {
        return Ok(false);
    }

    let suffix = &class["grid-rows-".len()..];

    if let Some(val) = parse_arbitrary_grid_template(suffix) {
        deny_computed_style!(ctx);
        insert_picking_style!(ctx, GridTemplateRows, val);
        ctx.insert_node_prop(NodeProp::GridTemplateRows, val);

        return Ok(true);
    }

    match suffix {
        "none" => {
            let value = quote! {
                Default::default()
            };
            deny_computed_style!(ctx);
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
            deny_computed_style!(ctx);
            insert_picking_style!(ctx, GridTemplateRows, value);
            ctx.insert_node_prop(NodeProp::GridTemplateRows, value);

            Ok(true)
        }
    }
}

fn parse_arbitrary_grid_template(str: &str) -> Option<RepeatedGridTrack> {
    let str = if str.starts_with("[") && str.ends_with("]") {
        &str[1..str.len() - 1]
    } else {
        return None;
    };

    let mut tracks = vec![];
    for str in str.split("_") {
        if str == "auto" {
            tracks.push(GridTrack::Auto);
            continue;
        }
        if let Some(px) = parse_px(str) {
            tracks.push(GridTrack::Px(px));
            continue;
        }
        if let Some(percent) = parse_percent(str) {
            tracks.push(GridTrack::Percent(percent));
            continue;
        }

        if let Some(fr) = parse_fr(str) {
            tracks.push(GridTrack::Fr(fr));
            continue;
        }

        if str == "min-content" {
            tracks.push(GridTrack::MinContent);
            continue;
        }

        if str == "max-content" {
            tracks.push(GridTrack::MaxContent);
            continue;
        }

        if str.starts_with("minmax(") && str.ends_with(")") {
            let str = &str["minmax(".len()..str.len() - 1];
            let mut parts = str.split(",");
            let min = parts.next()?;
            let max = parts.next()?;

            if parts.next().is_some() {
                return None;
            }

            let min = parse_min_track_sizing_function(min)?;
            let max = parse_max_track_sizing_function(max)?;
            tracks.push(GridTrack::Minmax(min, max));
            continue;
        }

        return None;
    }

    Some(RepeatedGridTrack(tracks))
}

fn parse_min_track_sizing_function(str: &str) -> Option<MinTrackSizingFunction> {
    if str == "0" {
        return Some(MinTrackSizingFunction::Px(0.));
    }

    if let Some(px) = parse_px(str) {
        return Some(MinTrackSizingFunction::Px(px));
    }

    todo!("parse_min_track_sizing_function")
}

fn parse_max_track_sizing_function(str: &str) -> Option<MaxTrackSizingFunction> {
    if str == "0" {
        return Some(MaxTrackSizingFunction::Px(0.));
    }

    if let Some(px) = parse_px(str) {
        return Some(MaxTrackSizingFunction::Px(px));
    }

    if let Some(fr) = parse_fr(str) {
        return Some(MaxTrackSizingFunction::Fraction(fr));
    }

    todo!("parse_max_track_sizing_function")
}

fn parse_fr(str: &str) -> Option<f32> {
    if str.ends_with("fr") {
        let str = &str[..str.len() - 2];
        let fr = str.parse::<f32>().ok()?;
        if fr <= 0. {
            return None;
        }
        return Some(fr);
    }

    None
}

struct RepeatedGridTrack(Vec<GridTrack>);

impl ToTokenStream for RepeatedGridTrack {
    fn to_token_stream(&self) -> TokenStream {
        let tracks = self.0.iter().map(|track| track.to_token_stream());
        quote! {
            vec![#(#tracks),*]
        }
    }
}

#[derive(Clone, Copy)]
#[expect(dead_code)]
enum GridTrack {
    Px(f32),
    Percent(f32),
    Auto,
    Fr(f32),
    MinContent,
    MaxContent,
    FitContentPx(f32),
    FitContentPercent(f32),
    Minmax(MinTrackSizingFunction, MaxTrackSizingFunction),
    VMin(f32),
    VMax(f32),
    Vh(f32),
    Vw(f32),
}

impl ToTokenStream for GridTrack {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            GridTrack::Px(val) => quote! { bevy::ui::GridTrack::px(#val) },
            GridTrack::Percent(val) => quote! { bevy::ui::GridTrack::percent(#val) },
            GridTrack::Auto => quote! { bevy::ui::GridTrack::auto() },
            GridTrack::Fr(val) => quote! { bevy::ui::GridTrack::fr(#val) },
            GridTrack::MinContent => quote! { bevy::ui::GridTrack::min_content() },
            GridTrack::MaxContent => quote! { bevy::ui::GridTrack::max_content() },
            GridTrack::FitContentPx(_) => todo!("GridTrack::FitContentPx"),
            GridTrack::FitContentPercent(_) => todo!("GridTrack::FitContentPercent"),
            GridTrack::Minmax(min, max) => {
                let min = min.to_token_stream();
                let max = max.to_token_stream();
                quote! {
                    bevy::ui::GridTrack::minmax(
                        #min,
                        #max
                    )
                }
            }
            GridTrack::VMin(_) => todo!("GridTrack::VMin"),
            GridTrack::VMax(_) => todo!("GridTrack::VMax"),
            GridTrack::Vh(_) => todo!("GridTrack::Vh"),
            GridTrack::Vw(_) => todo!("GridTrack::Vw"),
        }
    }
}

#[derive(Clone, Copy)]
#[expect(dead_code)]
enum MinTrackSizingFunction {
    Px(f32),
    Percent(f32),
    MinContent,
    MaxContent,
    Auto,
    VMin(f32),
    VMax(f32),
    Vh(f32),
    Vw(f32),
}

impl ToTokenStream for MinTrackSizingFunction {
    fn to_token_stream(&self) -> TokenStream {
        match self {
            MinTrackSizingFunction::Px(val) => {
                quote! { bevy::ui::MinTrackSizingFunction::Px(#val) }
            }
            MinTrackSizingFunction::Percent(val) => {
                quote! { bevy::ui::MinTrackSizingFunction::Percent(#val) }
            }
            MinTrackSizingFunction::MinContent => {
                quote! { bevy::ui::MinTrackSizingFunction::MinContent }
            }
            MinTrackSizingFunction::MaxContent => {
                quote! { bevy::ui::MinTrackSizingFunction::MaxContent }
            }
            MinTrackSizingFunction::Auto => quote! { bevy::ui::MinTrackSizingFunction::Auto },
            MinTrackSizingFunction::VMin(val) => {
                quote! { bevy::ui::MinTrackSizingFunction::VMin(#val) }
            }
            MinTrackSizingFunction::VMax(val) => {
                quote! { bevy::ui::MinTrackSizingFunction::VMax(#val) }
            }
            MinTrackSizingFunction::Vh(val) => {
                quote! { bevy::ui::MinTrackSizingFunction::Vh(#val) }
            }
            MinTrackSizingFunction::Vw(val) => {
                quote! { bevy::ui::MinTrackSizingFunction::Vw(#val) }
            }
        }
    }
}

#[derive(Clone, Copy)]
#[expect(dead_code)]
enum MaxTrackSizingFunction {
    Px(f32),
    Percent(f32),
    MinContent,
    MaxContent,
    FitContentPx(f32),
    FitContentPercent(f32),
    Auto,
    Fraction(f32),
    VMin(f32),
    VMax(f32),
    Vh(f32),
    Vw(f32),
}

impl ToTokenStream for MaxTrackSizingFunction {
    fn to_token_stream(&self) -> TokenStream {
        match self {
            MaxTrackSizingFunction::Px(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::Px(#val) }
            }
            MaxTrackSizingFunction::Percent(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::Percent(#val) }
            }
            MaxTrackSizingFunction::MinContent => {
                quote! { bevy::ui::MaxTrackSizingFunction::MinContent }
            }
            MaxTrackSizingFunction::MaxContent => {
                quote! { bevy::ui::MaxTrackSizingFunction::MaxContent }
            }
            MaxTrackSizingFunction::FitContentPx(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::FitContentPx(#val) }
            }
            MaxTrackSizingFunction::FitContentPercent(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::FitContentPercent(#val) }
            }
            MaxTrackSizingFunction::Auto => {
                quote! { bevy::ui::MaxTrackSizingFunction::Auto }
            }
            MaxTrackSizingFunction::Fraction(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::Fraction(#val) }
            }
            MaxTrackSizingFunction::VMin(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::VMin(#val) }
            }
            MaxTrackSizingFunction::VMax(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::VMax(#val) }
            }
            MaxTrackSizingFunction::Vh(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::Vh(#val) }
            }
            MaxTrackSizingFunction::Vw(val) => {
                quote! { bevy::ui::MaxTrackSizingFunction::Vw(#val) }
            }
        }
    }
}
