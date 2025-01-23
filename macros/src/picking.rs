use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    node::NodeProp,
    utils::quote::{Quote, QuoteCtx, Struct, StructVal, ToTokenStream},
    ParseCtx,
};

pub struct PickingStyles {
    path: TokenStream,
    hover: Struct<PickingStyleProp>,
    focus: Struct<PickingStyleProp>,
}

impl PickingStyles {
    pub fn path(&self) -> &TokenStream {
        &self.path
    }
}

impl Default for PickingStyles {
    fn default() -> Self {
        PickingStyles {
            path: quote! {bevy_tailwind::PickingStyles},
            hover: Struct::new(quote! { bevy_tailwind::PickingStyle}),
            focus: Struct::new(quote! { bevy_tailwind::PickingStyle}),
        }
    }
}

impl Quote for PickingStyles {
    fn quote(&self, ctx: &mut QuoteCtx) -> TokenStream {
        if self.focus.props.is_empty() && self.hover.props.is_empty() {
            return TokenStream::new();
        }

        let base = create_base_style(self, ctx);
        ctx.parent_props.push("hover".to_string());
        let hover = self.hover.quote(ctx);
        ctx.parent_props.pop();
        let hover = if hover.is_empty() || !ctx.is_create {
            hover
        } else {
            quote! {hover: #hover,}
        };
        ctx.parent_props.push("focus".to_string());
        let focus = self.focus.quote(ctx);
        ctx.parent_props.pop();
        let focus = if focus.is_empty() || !ctx.is_create {
            focus
        } else {
            quote! {focus: #focus,}
        };
        if ctx.is_create {
            return quote! {
                bevy_tailwind::PickingStyles {
                    base: #base,
                    #hover
                    #focus
                    ..Default::default()
                }
            };
        }

        quote! {
            #base
            #hover
            #focus
        }
    }
}

impl ParseCtx {
    pub fn insert_picking_style(&mut self, prop: PickingStyleProp, val: impl ToTokenStream) {
        if self.hover {
            let val = val.to_token_stream();
            let val = quote! {Some(#val)};
            self.components
                .picking_styles
                .hover
                .props
                .insert(prop, StructVal::prioritized(val, self.class_type, 0, false));
            return;
        }

        if self.focus {
            let val = val.to_token_stream();
            let val = quote! {Some(#val)};
            self.components
                .picking_styles
                .focus
                .props
                .insert(prop, StructVal::prioritized(val, self.class_type, 0, false));
        }
    }
}

fn create_base_style(picking_styles: &PickingStyles, ctx: &mut QuoteCtx) -> TokenStream {
    if !ctx.is_create {
        panic!("Picking style does not support mutation yet.");
    }

    let mut base = Struct::new(quote! { bevy_tailwind::PickingStyle });
    ctx.parent_props.push("base".to_string());
    macro_rules! _insert_prop {
        ($picking_prop:ident, $ty_path:expr, $prop:expr) => {
            if !ctx.is_create
                || picking_styles
                    .hover
                    .props
                    .contains_key(&PickingStyleProp::$picking_prop)
                || picking_styles
                    .focus
                    .props
                    .contains_key(&PickingStyleProp::$picking_prop)
            {
                if let Some(prop) = $prop {
                    let prop = if ctx.is_create {
                        let prop = prop.quote(ctx);
                        quote! {Some(#prop)}
                    } else {
                        // unreachable as mutation is not supported
                        ctx.parent_props
                            .push(PickingStyleProp::$picking_prop.as_ref().to_string());
                        let prop = prop.quote(ctx);
                        ctx.parent_props.pop();

                        // dirty hack to turn T into Option<T>
                        prop.to_string()
                            .replace(";", ".into();")
                            .parse::<TokenStream>()
                            .unwrap()
                    };

                    base.props
                        .insert(&PickingStyleProp::$picking_prop, StructVal::raw(prop));
                } else if ctx.is_create {
                    let prop = quote! {Some($ty_path::default())};
                    base.props
                        .insert(&PickingStyleProp::$picking_prop, StructVal::raw(prop));
                }
            }
        };
    }
    macro_rules! insert_prop {
        ($picking_prop:ident, $ty_path:expr, $comp:ident, $prop:expr) => {
            _insert_prop!(
                $picking_prop,
                $ty_path,
                ctx.parse_ctx.components.$comp.props.get($prop)
            );
        };
        ($picking_prop:ident, $ty_path:expr, $comp:ident, $prop:expr, $prop2:expr) => {
            _insert_prop!(
                $picking_prop,
                $ty_path,
                ctx.parse_ctx
                    .components
                    .$comp
                    .props
                    .get($prop)
                    .and_then(|p| p.as_nested().props.get($prop2))
            );
        };
    }

    #[rustfmt::skip]
    insert_prop!(AspectRatio, bevy::ui::AspectRatio, node, &NodeProp::AspectRatio);
    insert_prop!(Display, bevy::ui::Display, node, &NodeProp::Display);
    #[rustfmt::skip]
    insert_prop!(OverflowX, bevy::ui::Overflow, node, &NodeProp::Overflow, "x");
    #[rustfmt::skip]
    insert_prop!(OverflowY, bevy::ui::Overflow, node, &NodeProp::Overflow, "y");
    #[rustfmt::skip]
    insert_prop!(Position, bevy::ui::PositionType, node, &NodeProp::PositionType);
    insert_prop!(Top, bevy::ui::Val, node, &NodeProp::Top);
    insert_prop!(Right, bevy::ui::Val, node, &NodeProp::Right);
    insert_prop!(Bottom, bevy::ui::Val, node, &NodeProp::Bottom);
    insert_prop!(Left, bevy::ui::Val, node, &NodeProp::Left);
    insert_prop!(FlexBasis, bevy::ui::Val, node, &NodeProp::FlexBasis);
    #[rustfmt::skip]
    insert_prop!(FlexDirection, bevy::ui::FlexDirection ,node, &NodeProp::FlexDirection);
    insert_prop!(FlexWrap, bevy::ui::FlexWrap, node, &NodeProp::FlexWrap);
    insert_prop!(FlexGrow, bevy::ui::Val, node, &NodeProp::FlexGrow);
    insert_prop!(FlexShrink, bevy::ui::Val, node, &NodeProp::FlexShrink);
    #[rustfmt::skip]
    insert_prop!(GridTemplateColumns, Vec, node, &NodeProp::GridTemplateColumns);
    #[rustfmt::skip]
    insert_prop!(GridTemplateRows, Vec, node, &NodeProp::GridTemplateRows);
    #[rustfmt::skip]
    insert_prop!(GridAutoFlow, bevy::ui::GridAutoFlow, node, &NodeProp::GridAutoFlow);
    #[rustfmt::skip]
    insert_prop!(GridAutoColumns, bevy::ui::GridAuto, node, &NodeProp::GridAutoColumns);
    #[rustfmt::skip]
    insert_prop!(GridAutoRows, bevy::ui::GridAuto, node, &NodeProp::GridAutoRows);
    insert_prop!(ColumnGap, bevy::ui::Val, node, &NodeProp::ColumnGap);
    insert_prop!(RowGap, bevy::ui::Val, node, &NodeProp::RowGap);
    #[rustfmt::skip]
    insert_prop!(JustifyContent, bevy::ui::JustifyContent, node, &NodeProp::JustifyContent);
    #[rustfmt::skip]
    insert_prop!(JustifyItems, bevy::ui::JustifyItems, node, &NodeProp::JustifyItems);
    #[rustfmt::skip]
    insert_prop!(JustifySelf, bevy::ui::JustifySelf, node, &NodeProp::JustifySelf);
    #[rustfmt::skip]
    insert_prop!(AlignContent, bevy::ui::AlignContent, node, &NodeProp::AlignContent);
    #[rustfmt::skip]
    insert_prop!(AlignItems, bevy::ui::AlignItems, node, &NodeProp::AlignItems);
    insert_prop!(AlignSelf, bevy::ui::AlignSelf, node, &NodeProp::AlignSelf);
    insert_prop!(PaddingTop, bevy::ui::Val, node, &NodeProp::Padding, "top");
    #[rustfmt::skip]
    insert_prop!(PaddingRight, bevy::ui::Val, node, &NodeProp::Padding, "right");
    #[rustfmt::skip]
    insert_prop!(PaddingBottom, bevy::ui::Val, node, &NodeProp::Padding, "bottom");
    insert_prop!(PaddingLeft, bevy::ui::Val, node, &NodeProp::Padding, "left");
    insert_prop!(MarginTop, bevy::ui::Val, node, &NodeProp::Margin, "top");
    insert_prop!(MarginRight, bevy::ui::Val, node, &NodeProp::Margin, "right");
    #[rustfmt::skip]
    insert_prop!(MarginBottom, bevy::ui::Val, node, &NodeProp::Margin, "bottom");
    insert_prop!(MarginLeft, bevy::ui::Val, node, &NodeProp::Margin, "left");
    insert_prop!(Width, bevy::ui::Val, node, &NodeProp::Width);
    insert_prop!(MinWidth, bevy::ui::Val, node, &NodeProp::MinWidth);
    insert_prop!(MaxWidth, bevy::ui::Val, node, &NodeProp::MaxWidth);
    insert_prop!(Height, bevy::ui::Val, node, &NodeProp::Height);
    insert_prop!(MinHeight, bevy::ui::Val, node, &NodeProp::MinHeight);
    insert_prop!(MaxHeight, bevy::ui::Val, node, &NodeProp::MaxHeight);

    insert_prop!(FontSize, bevy::ui::FontSize, text_font, "font_size");
    #[rustfmt::skip]
    insert_prop!(TextJustify, bevy::text::JustifyText, text_layout, "justify");
    #[rustfmt::skip]
    insert_prop!(TextLinebreak, bevy::text::LineBreak, text_layout, "linebreak");

    insert_prop!(TextColor, bevy::color::Color, text_color, "0");

    insert_prop!(BackgroundColor, bevy::color::Color, background_color, "0");

    insert_prop!(ZIndex, i32, z_index, "0");

    insert_prop!(BorderRadiusTl, bevy::ui::Val, border_radius, "top_left");
    insert_prop!(BorderRadiusTr, bevy::ui::Val, border_radius, "top_right");
    insert_prop!(BorderRadiusBr, bevy::ui::Val, border_radius, "bottom_right");
    insert_prop!(BorderRadiusBl, bevy::ui::Val, border_radius, "bottom_left");

    #[rustfmt::skip]
    insert_prop!(BorderTop, bevy::ui::Border, node, &NodeProp::Border, "top");
    #[rustfmt::skip]
    insert_prop!(BorderRight, bevy::ui::Border, node, &NodeProp::Border, "right");
    #[rustfmt::skip]
    insert_prop!(BorderBottom, bevy::ui::Border, node, &NodeProp::Border, "bottom");
    #[rustfmt::skip]
    insert_prop!(BorderLeft, bevy::ui::Border, node, &NodeProp::Border, "left");

    insert_prop!(BorderColor, bevy::color::Color, border_color, "0");

    insert_prop!(OutlineWidth, bevy::ui::Val, outline, "width");

    insert_prop!(OutlineColor, bevy::color::Color, outline, "color");
    insert_prop!(OutlineOffset, bevy::ui::Val, outline, "offset");

    let base = base.quote(ctx);
    ctx.parent_props.pop();

    base
}

macro_rules! insert_picking_style {
    ($ctx:ident, $prop:ident, $value:expr) => {
        if $ctx.hover || $ctx.focus {
            $ctx.insert_picking_style(crate::picking::PickingStyleProp::$prop, $value);
            return Ok(true);
        }
    };
}

pub(crate) use insert_picking_style;

macro_rules! deny_picking_style {
    ($ctx:ident) => {
        if $ctx.hover || $ctx.focus {
            return Err(crate::ParseClassError::Unknown);
        }
    };
}

pub(crate) use deny_picking_style;

pub fn parse_picking_class(class: &str) -> (bool, bool, &str) {
    if class.starts_with("hover:") {
        return (true, false, &class[6..]);
    }

    if class.starts_with("focus:") {
        return (false, true, &class[6..]);
    }

    (false, false, class)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PickingStyleProp {
    AspectRatio,
    Display,
    OverflowX,
    OverflowY,
    Position,
    Top,
    Right,
    Bottom,
    Left,
    ZIndex,
    FlexBasis,
    FlexDirection,
    FlexWrap,
    FlexGrow,
    FlexShrink,
    GridTemplateColumns,
    // GridColumn,
    GridTemplateRows,
    // GridRow,
    GridAutoFlow,
    GridAutoColumns,
    GridAutoRows,
    ColumnGap,
    RowGap,
    JustifyContent,
    JustifyItems,
    JustifySelf,
    AlignContent,
    AlignItems,
    AlignSelf,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
    FontSize,
    TextJustify,
    TextColor,
    TextLinebreak,
    BackgroundColor,
    BorderRadiusTl,
    BorderRadiusTr,
    BorderRadiusBr,
    BorderRadiusBl,
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
    BorderColor,
    OutlineWidth,
    OutlineColor,
    OutlineOffset,
}
impl AsRef<str> for PickingStyleProp {
    fn as_ref(&self) -> &str {
        match self {
            PickingStyleProp::AspectRatio => "aspect_ratio",
            PickingStyleProp::Display => "display",
            PickingStyleProp::OverflowX => "overflow_x",
            PickingStyleProp::OverflowY => "overflow_y",
            PickingStyleProp::Position => "position",
            PickingStyleProp::Top => "top",
            PickingStyleProp::Right => "right",
            PickingStyleProp::Bottom => "bottom",
            PickingStyleProp::Left => "left",
            PickingStyleProp::ZIndex => "z_index",
            PickingStyleProp::FlexBasis => "flex_basis",
            PickingStyleProp::FlexDirection => "flex_direction",
            PickingStyleProp::FlexWrap => "flex_wrap",
            PickingStyleProp::FlexGrow => "flex_grow",
            PickingStyleProp::FlexShrink => "flex_shrink",
            PickingStyleProp::GridTemplateColumns => "grid_template_columns",
            // PickingStyleProp::GridColumn => "grid_column",
            PickingStyleProp::GridTemplateRows => "grid_template_rows",
            // PickingStyleProp::GridRow => "grid_row",
            PickingStyleProp::GridAutoFlow => "grid_auto_flow",
            PickingStyleProp::GridAutoColumns => "grid_auto_columns",
            PickingStyleProp::GridAutoRows => "grid_auto_rows",
            PickingStyleProp::ColumnGap => "column_gap",
            PickingStyleProp::RowGap => "row_gap",
            PickingStyleProp::JustifyContent => "justify_content",
            PickingStyleProp::JustifyItems => "justify_items",
            PickingStyleProp::JustifySelf => "justify_self",
            PickingStyleProp::AlignContent => "align_content",
            PickingStyleProp::AlignItems => "align_items",
            PickingStyleProp::AlignSelf => "align_self",
            PickingStyleProp::PaddingTop => "padding_top",
            PickingStyleProp::PaddingRight => "padding_right",
            PickingStyleProp::PaddingBottom => "padding_bottom",
            PickingStyleProp::PaddingLeft => "padding_left",
            PickingStyleProp::MarginTop => "margin_top",
            PickingStyleProp::MarginRight => "margin_right",
            PickingStyleProp::MarginBottom => "margin_bottom",
            PickingStyleProp::MarginLeft => "margin_left",
            PickingStyleProp::Width => "width",
            PickingStyleProp::MinWidth => "min_width",
            PickingStyleProp::MaxWidth => "max_width",
            PickingStyleProp::Height => "height",
            PickingStyleProp::MinHeight => "min_height",
            PickingStyleProp::MaxHeight => "max_height",
            PickingStyleProp::FontSize => "font_size",
            PickingStyleProp::TextJustify => "text_justity",
            PickingStyleProp::TextColor => "text_color",
            PickingStyleProp::TextLinebreak => "text_linebreak",
            PickingStyleProp::BackgroundColor => "background_color",
            PickingStyleProp::BorderRadiusTl => "border_radius_tl",
            PickingStyleProp::BorderRadiusTr => "border_radius_tr",
            PickingStyleProp::BorderRadiusBr => "border_radius_br",
            PickingStyleProp::BorderRadiusBl => "border_radius_bl",
            PickingStyleProp::BorderTop => "border_top",
            PickingStyleProp::BorderRight => "border_right",
            PickingStyleProp::BorderBottom => "border_bottom",
            PickingStyleProp::BorderLeft => "border_left",
            PickingStyleProp::BorderColor => "border_color",
            PickingStyleProp::OutlineWidth => "outline_width",
            PickingStyleProp::OutlineColor => "outline_color",
            PickingStyleProp::OutlineOffset => "outline_offset",
        }
    }
}
