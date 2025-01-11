use crate::{ParseCtx, ParseResult, utils::quote::Quote};

// mod box_sizing;
mod align_content;
mod align_items;
mod align_self;
mod aspect_ratio;
mod border;
mod display;
mod flex;
mod flex_basis;
mod flex_direction;
mod flex_grow;
mod flex_shrink;
mod flex_wrap;
mod gap;
mod grid_auto_columns;
mod grid_auto_flow;
mod grid_auto_rows;
mod grid_column;
mod grid_row;
mod grid_template_columns;
mod grid_template_rows;
mod justify_content;
mod justify_items;
mod justity_self;
mod overflow;
mod place_content;
mod place_items;
mod place_self;
mod position_type;
mod size;
mod spacing;
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
    pub fn parse_node(&mut self, class: &str) -> ParseResult {
        parse_class!(
            // box_sizing::parse_box_sizing(self, class),
            aspect_ratio::parse_aspect_ratio(self, class),
            display::parse_display(self, class),
            overflow::parse_overflow(self, class),
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
            grid_row::parse_grid_row(self, class),
            grid_auto_flow::parse_grid_auto_flow(self, class),
            grid_auto_columns::parse_grid_auto_columns(self, class),
            grid_auto_rows::parse_grid_auto_rows(self, class),
            gap::parse_gap(self, class),
            justify_content::parse_justify_content(self, class),
            justify_items::parse_justify_items(self, class),
            justity_self::parse_justify_self(self, class),
            align_content::parse_align_content(self, class),
            align_items::parse_align_items(self, class),
            align_self::parse_align_self(self, class),
            place_content::parse_place_content(self, class),
            place_items::parse_place_items(self, class),
            place_self::parse_place_self(self, class),
            spacing::parse_padding(self, class),
            spacing::parse_margin(self, class),
            size::parse_width(self, class),
            size::parse_min_width(self, class),
            size::parse_max_width(self, class),
            size::parse_height(self, class),
            size::parse_min_height(self, class),
            size::parse_max_height(self, class),
            size::parse_size(self, class),
            border::parse_border(self, class)
        );

        return Ok(false);
    }

    fn insert_node_prop(&mut self, prop: NodeProp, value: impl Quote + 'static) {
        self.insert_node_prop_priority(prop, value, 0);
    }

    fn insert_node_prop_priority(
        &mut self,
        prop: NodeProp,
        value: impl Quote + 'static,
        priority: u8,
    ) {
        self.components
            .node
            .insert(prop, value, self.class_type, priority);
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum NodeProp {
    Display,
    // BoxSizing,
    PositionType,
    Overflow,
    // OverflowClipMargin,
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
            // NodeProp::OverflowClipMargin => "overflow_clip_margin",
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

macro_rules! insert_node_prop_nested {
    ($ctx:ident, $node_prop:expr, $path:expr, $value:expr, $priority:literal, $props:expr, $use_setter:literal) => {
        let s = $ctx
            .components
            .node
            .props
            .entry($node_prop)
            .or_insert_with(|| {
                crate::utils::quote::StructVal::nested(
                    crate::utils::quote::Struct::<&'static str>::new($path).use_setter($use_setter),
                )
            })
            .as_nested_mut();

        for prop in $props {
            if let Some(prop) = s.props.get_mut(prop) {
                prop.as_priotized_mut()
                    .insert($value, $ctx.class_type, $priority);
            } else {
                s.insert(prop, $value, $ctx.class_type, $priority);
            }
        }

        return Ok(true);
    };

    ($ctx:ident, $node_prop:expr, $path:expr, $value:expr, $priority:literal, $props:expr) => {
        crate::node::insert_node_prop_nested!(
            $ctx, $node_prop, $path, $value, $priority, $props, false
        );
    };
}

pub(crate) use insert_node_prop_nested;

macro_rules! insert_grid_placement_props {
    ($ctx:ident, $node_prop:expr, $value:expr, $priority:literal, $props:expr) => {
        crate::node::insert_node_prop_nested!(
            $ctx,
            $node_prop,
            quote::quote! {bevy::ui::GridPlacement},
            $value,
            $priority,
            $props,
            true
        );
    };
}

pub(crate) use insert_grid_placement_props;

macro_rules! insert_node_ui_rect {
    ($ctx:ident, $node_prop:expr, $value:expr, $priority:literal, $props:expr) => {
        crate::node::insert_node_prop_nested!(
            $ctx,
            $node_prop,
            quote::quote! {bevy::ui::UiRect},
            $value,
            $priority,
            $props
        );
    };
}

pub(crate) use insert_node_ui_rect;
