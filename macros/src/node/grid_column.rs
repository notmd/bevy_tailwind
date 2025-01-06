use std::num::NonZero;

use crate::{ParseClassError, ParseCtx, ParseResult, utils::StructPropValue};

use super::{GridPlacement, NodeProp};

pub fn parse_grid_column(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("col-") {
        return Ok(false);
    }

    let class = class["col-".len()..].to_string();

    if class == "auto" {
        ctx.components.node.insert(
            NodeProp::GridColumn,
            StructPropValue::nested(ctx.class_type, GridPlacement::default()),
        );
        return Ok(true);
    }

    if class.starts_with("span-") {
        let class = &class["span-".len()..];
        if class == "full" {
            let grid_placement = ctx
                .components
                .node
                .entry(NodeProp::GridColumn)
                .or_insert_with(|| {
                    StructPropValue::nested(ctx.class_type, GridPlacement::default())
                })
                .value
                .downcast_mut::<GridPlacement>();

            grid_placement.start = Some(NonZero::new(1).unwrap());
            grid_placement.end = Some(NonZero::new(-1).unwrap());
            return Ok(true);
        } else {
            let span = class.parse::<NonZero<u16>>();

            match span {
                Ok(span) => {
                    ctx.components
                        .node
                        .entry(NodeProp::GridColumn)
                        .or_insert_with(|| {
                            StructPropValue::nested(ctx.class_type, GridPlacement::default())
                        })
                        .value
                        .downcast_mut::<GridPlacement>()
                        .span = Some(span);

                    return Ok(true);
                }
                Err(_) => {
                    return Err(ParseClassError::Unsupported);
                }
            }
        }
    }

    if class.starts_with("start-") {
        let class = &class["start-".len()..];
        let start = if class == "auto" {
            NonZero::new(1).unwrap()
        } else {
            let Ok(start) = class.parse::<NonZero<i16>>() else {
                return Err(ParseClassError::Unsupported);
            };
            start
        };

        ctx.components
            .node
            .entry(NodeProp::GridColumn)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, GridPlacement::default()))
            .value
            .downcast_mut::<GridPlacement>()
            .start = Some(start);

        return Ok(true);
    }

    if class.starts_with("end-") {
        let class = &class["end-".len()..];
        let end = if class == "auto" {
            NonZero::new(1).unwrap()
        } else {
            let Ok(end) = class.parse::<NonZero<i16>>() else {
                return Err(ParseClassError::Unsupported);
            };
            end
        };

        ctx.components
            .node
            .entry(NodeProp::GridColumn)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, GridPlacement::default()))
            .value
            .downcast_mut::<GridPlacement>()
            .end = Some(end);

        return Ok(true);
    }

    return Ok(false);
}
