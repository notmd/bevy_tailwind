use std::num::NonZero;

use crate::{
    node::insert_grid_placement_props,
    utils::{
        deny_computed_style,
        quote::{Struct, StructVal},
    },
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;
use quote::quote;

// TODO: find a way to support computed style
pub fn parse_grid_column(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("col-") {
        return Ok(false);
    }

    let class = class["col-".len()..].to_string();

    if class == "auto" {
        deny_computed_style!(ctx);
        insert_grid_placement_props!(ctx, NodeProp::GridColumn, quote! {1u16}, 0, ["set_span"]);
    }

    if class.starts_with("span-") {
        let class = &class["span-".len()..];
        if class == "full" {
            deny_computed_style!(ctx);
            let grid_placement = ctx
                .components
                .node
                .props
                .entry(NodeProp::GridColumn)
                .or_insert_with(|| {
                    StructVal::Nested(
                        Struct::new(quote! { bevy::ui::GridPlacement }).use_setter(true),
                    )
                })
                .as_nested_mut();

            grid_placement.insert("set_start", quote! {1i16}, &ctx.class_type, 0);
            grid_placement.insert("set_end", quote! {-1i16}, &ctx.class_type, 0);
            return Ok(true);
        } else {
            let span = class.parse::<NonZero<u16>>();

            match span {
                Ok(span) => {
                    deny_computed_style!(ctx);
                    let span = span.get();
                    insert_grid_placement_props!(
                        ctx,
                        NodeProp::GridColumn,
                        quote! {#span},
                        1,
                        ["set_span"]
                    );
                }
                Err(_) => {
                    return Err(ParseClassError::Unknown);
                }
            }
        }
    }

    if class.starts_with("start-") {
        let class = &class["start-".len()..];
        let start = if class == "auto" {
            1
        } else {
            let Ok(start) = class.parse::<NonZero<i16>>() else {
                return Err(ParseClassError::Unknown);
            };
            start.get()
        };
        deny_computed_style!(ctx);
        insert_grid_placement_props!(ctx, NodeProp::GridColumn, quote! {#start}, 1, ["set_start"]);
    }

    if class.starts_with("end-") {
        let class = &class["end-".len()..];
        let end = if class == "auto" {
            1
        } else {
            let Ok(end) = class.parse::<NonZero<i16>>() else {
                return Err(ParseClassError::Unknown);
            };
            end.get()
        };
        deny_computed_style!(ctx);
        insert_grid_placement_props!(ctx, NodeProp::GridColumn, quote! {#end}, 1, ["set_end"]);
    }

    Ok(false)
}
