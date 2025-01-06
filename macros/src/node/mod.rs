use std::num::NonZero;

use quote::quote;

use crate::{
    ParseCtx, ParseResult,
    utils::{StructPropValue, StructPropValueType, StructualTokenStream, ToTokenStream, val::Val},
};

// mod box_sizing;
mod display;
mod flex;
mod flex_basis;
mod flex_direction;
mod flex_grow;
mod flex_shrink;
mod flex_wrap;
mod grid_column;
mod grid_template_columns;
mod grid_template_rows;
mod padding;
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
            flex_shrink::parse_flex_shrink(self, class),
            grid_template_columns::parse_grid_template_columns(self, class),
            grid_column::parse_grid_column(self, class),
            grid_template_rows::parse_grid_template_rows(self, class),
            padding::parse_padding(self, class)
        );

        return Ok(false);
    }

    fn insert_node_prop_simple(&mut self, prop: NodeProp, value: impl ToTokenStream) {
        self.components.node.insert(prop, StructPropValue {
            class_type: self.class_type,
            value: StructPropValueType::Simple(value.to_token_stream()),
        });
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

#[derive(Default, Clone)]
struct UiRect {
    top: Option<Val>,
    right: Option<Val>,
    bottom: Option<Val>,
    left: Option<Val>,
}

impl ToTokenStream for UiRect {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let top = self.top.map(|v| {
            let v = v.to_token_stream();
            quote! {
                top: #v
            }
        });
        let right = self.right.map(|v| {
            let v = v.to_token_stream();
            quote! {
                right: #v
            }
        });
        let bottom = self.bottom.map(|v| {
            let v = v.to_token_stream();
            quote! {
                bottom: #v
            }
        });
        let left = self.left.map(|v| {
            let v = v.to_token_stream();
            quote! {
                left: #v
            }
        });

        let props = [top, right, bottom, left]
            .into_iter()
            .filter(Option::is_some);

        quote! {
            bevy::ui::UiRect {
                #(#props,)*
               ..Default::default()
            }
        }
    }

    fn structual_to_token_stream(&self) -> Option<crate::utils::StructualTokenStream> {
        let mut res = StructualTokenStream::default();
        if let Some(ref top) = self.top {
            res.push(("top".to_string(), top.to_token_stream()));
        }

        if let Some(ref right) = self.right {
            res.push(("right".to_string(), right.to_token_stream()));
        }

        if let Some(ref bottom) = self.bottom {
            res.push(("bottom".to_string(), bottom.to_token_stream()));
        }

        if let Some(ref left) = self.left {
            res.push(("left".to_string(), left.to_token_stream()));
        }

        Some(res)
    }

    fn as_any(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

struct GridPlacement {
    start: Option<NonZero<i16>>,
    span: Option<NonZero<u16>>,
    end: Option<NonZero<i16>>,
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self {
            start: None,
            span: None,
            end: None,
        }
    }
}

impl ToTokenStream for GridPlacement {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        if self.start.is_none() && self.span.is_none() && self.end.is_none() {
            return quote! {
                bevy::ui::GridPlacement::default()
            };
        }

        let start = self.start.clone().map(|v| {
            let v = v.get();
            quote! {
                .set_start(#v)
            }
        });
        let span = self.span.clone().map(|v| {
            let v = v.get();
            quote! {
                .set_span(#v)
            }
        });
        let end = self.end.clone().map(|v| {
            let v = v.get();
            quote! {
                .set_end(#v)
            }
        });
        quote! {
            bevy::ui::GridPlacement::default()
                #start
                #span
                #end
        }
    }

    fn structual_to_token_stream(&self) -> Option<crate::utils::StructualTokenStream> {
        if self.start.is_none() && self.span.is_none() && self.end.is_none() {
            return Some(StructualTokenStream(vec![
                ("set_start()".to_string(), quote! {0}),
                ("set_span()".to_string(), quote! {1}),
                ("set_end()".to_string(), quote! {0}),
            ]));
        }
        let mut res = StructualTokenStream::default();
        if let Some(ref start) = self.start {
            let start = start.get();
            res.push(("set_start()".to_string(), quote! {#start}));
        }

        if let Some(ref span) = self.span {
            let span = span.get();
            res.push(("set_span()".to_string(), quote! {#span}));
        }

        if let Some(ref end) = self.end {
            let end = end.get();
            res.push(("set_end()".to_string(), quote! {#end}));
        }

        Some(res)
    }

    fn as_any(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}
