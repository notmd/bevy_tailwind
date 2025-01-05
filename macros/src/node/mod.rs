use proc_macro2::{Span, TokenStream};
use syn::Ident;

use crate::{MacroType, ParseCtx, ParseResult};
use quote::quote;

// mod box_sizing;
mod display;
mod flex;
mod flex_basis;
mod flex_direction;
mod flex_grow;
mod flex_shrink;
mod flex_wrap;
mod position_type;
mod trbl;

macro_rules! parse_class {
    ($($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    return Ok(true)
                }
                Err(e) => {
                  return Err(e);
                }
                _ => {}
            }
        )*
    };
}

impl ParseCtx {
    pub fn parse_node_class(&mut self, class: &str) -> ParseResult {
        parse_class!(
            // box_sizing::parse_box_sizing(self, class),
            display::parse_display(self, class),
            position_type::parse_position_type(self, class),
            trbl::parse_trbl(self, class),
            flex_basis::parse_flex_basis(self, class),
            flex_direction::parse_flex_direction(self, class),
            flex_wrap::parse_flex_wrap(self, class),
            flex::parse_flex(self, class),
            flex_grow::parse_flex_grow(self, class),
            flex_shrink::parse_flex_shrink(self, class)
        );

        return Ok(false);
    }

    fn insert_node_prop(&mut self, prop: NodeProp, value: TokenStream) {
        self.components.node.insert(prop, (value, self.class_type));
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum NodeProp {
    Display,
    // BoxSizing,
    PositionType,
    Overflow,
    OverflowClipMargin,
    Left,
    Right,
    Top,
    Bottom,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    AspectRatio,
    AlignItems,
    JustifyItems,
    AlignSelf,
    JustifySelf,
    AlignContent,
    JustifyContent,
    Margin,
    Padding,
    Border,
    FlexDirection,
    FlexWrap,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    RowGap,
    ColumnGap,
    GridAutoFlow,
    GridTemplateRows,
    GridTemplateColumns,
    GridAutoRows,
    GridAutoColumns,
    GridRow,
    GridColumn,
}

impl AsRef<str> for NodeProp {
    fn as_ref(&self) -> &str {
        match self {
            NodeProp::Display => "display",
            // NodeProp::BoxSizing => "box_sizing",
            NodeProp::PositionType => "position_type",
            NodeProp::Overflow => "overflow",
            NodeProp::OverflowClipMargin => "overflow_clip_margin",
            NodeProp::Left => "left",
            NodeProp::Right => "right",
            NodeProp::Top => "top",
            NodeProp::Bottom => "bottom",
            NodeProp::Width => "width",
            NodeProp::Height => "height",
            NodeProp::MinWidth => "min_width",
            NodeProp::MinHeight => "min_height",
            NodeProp::MaxWidth => "max_width",
            NodeProp::MaxHeight => "max_height",
            NodeProp::AspectRatio => "aspect_ratio",
            NodeProp::AlignItems => "align_items",
            NodeProp::JustifyItems => "justify_items",
            NodeProp::AlignSelf => "align_self",
            NodeProp::JustifySelf => "justify_self",
            NodeProp::AlignContent => "align_content",
            NodeProp::JustifyContent => "justify_content",
            NodeProp::Margin => "margin",
            NodeProp::Padding => "padding",
            NodeProp::Border => "border",
            NodeProp::FlexDirection => "flex_direction",
            NodeProp::FlexWrap => "flex_wrap",
            NodeProp::FlexGrow => "flex_grow",
            NodeProp::FlexShrink => "flex_shrink",
            NodeProp::FlexBasis => "flex_basis",
            NodeProp::RowGap => "row_gap",
            NodeProp::ColumnGap => "column_gap",
            NodeProp::GridAutoFlow => "grid_auto_flow",
            NodeProp::GridTemplateRows => "grid_template_rows",
            NodeProp::GridTemplateColumns => "grid_template_columns",
            NodeProp::GridAutoRows => "grid_auto_rows",
            NodeProp::GridAutoColumns => "grid_auto_columns",
            NodeProp::GridRow => "grid_row",
            NodeProp::GridColumn => "grid_column",
        }
    }
}
