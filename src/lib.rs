use bevy::prelude::*;
pub use bevy_tailwind_macro::tw;

pub struct TailwindPlugin;

impl Plugin for TailwindPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, apply_picking_style);
    }
}

fn apply_picking_style(
    mut query: Query<
        (
            &PickingStyles,
            Ref<Interaction>,
            Option<&mut Node>,
            Option<&mut BackgroundColor>,
            Option<&mut ZIndex>,
            Option<&mut BorderRadius>,
            Option<&mut BorderColor>,
            Option<&mut Outline>,
            Option<&mut TextFont>,
            Option<&mut TextLayout>,
            Option<&mut TextColor>,
        ),
        Changed<Interaction>,
    >,
) {
    for (
        picking_styles,
        interaction,
        node,
        background_color,
        z_index,
        border_radius,
        border_color,
        outline,
        text_font,
        text_layout,
        text_color,
    ) in query.iter_mut()
    {
        if interaction.is_added() {
            continue;
        }

        fn apply_style(
            picking: &PickingStyle,
            node: Option<Mut<Node>>,
            background_color: Option<Mut<BackgroundColor>>,
            z_index: Option<Mut<ZIndex>>,
            border_radius: Option<Mut<BorderRadius>>,
            border_color: Option<Mut<BorderColor>>,
            outline: Option<Mut<Outline>>,
            text_font: Option<Mut<TextFont>>,
            text_layout: Option<Mut<TextLayout>>,
            text_color: Option<Mut<TextColor>>,
        ) {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(
                        if let Some(prop) = &picking.$picking_prop {
                            $lhs = prop.clone();
                        }
                    )+
                };
            }

            if let Some(mut node) = node {
                apply_style!(
                    (aspect_ratio, node.aspect_ratio),
                    (display, node.display),
                    (overflow_x, node.overflow.x),
                    (overflow_y, node.overflow.y),
                    (position, node.position_type),
                    (top, node.top),
                    (right, node.right),
                    (bottom, node.bottom),
                    (left, node.left),
                    (flex_basis, node.flex_basis),
                    (flex_direction, node.flex_direction),
                    (flex_wrap, node.flex_wrap),
                    (flex_grow, node.flex_grow),
                    (flex_shrink, node.flex_shrink),
                    (grid_template_columns, node.grid_template_columns),
                    // (grid_column, node.grid_column),
                    (grid_template_rows, node.grid_template_rows),
                    // (grid_row, node.grid_row),
                    (grid_auto_flow, node.grid_auto_flow),
                    (grid_auto_columns, node.grid_auto_columns),
                    (grid_auto_rows, node.grid_auto_rows),
                    (column_gap, node.column_gap),
                    (row_gap, node.row_gap),
                    (justify_content, node.justify_content),
                    (justify_items, node.justify_items),
                    (justify_self, node.justify_self),
                    (align_content, node.align_content),
                    (align_items, node.align_items),
                    (align_self, node.align_self),
                    (padding_top, node.padding.top),
                    (padding_right, node.padding.right),
                    (padding_bottom, node.padding.bottom),
                    (padding_left, node.padding.left),
                    (margin_top, node.margin.top),
                    (margin_right, node.margin.right),
                    (margin_bottom, node.margin.bottom),
                    (margin_left, node.margin.left),
                    (width, node.width),
                    (min_width, node.min_width),
                    (max_width, node.max_width),
                    (height, node.height),
                    (min_height, node.min_height),
                    (max_height, node.max_height),
                    (border_top, node.border.top),
                    (border_right, node.border.right),
                    (border_bottom, node.border.bottom),
                    (border_left, node.border.left)
                );
            }

            if let Some(mut background_color) = background_color {
                apply_style!((background_color, background_color.0));
            }

            if let Some(mut z_index) = z_index {
                apply_style!((z_index, z_index.0));
            }

            if let Some(mut border_radius) = border_radius {
                apply_style!(
                    (border_radius_tl, border_radius.top_left),
                    (border_radius_tr, border_radius.top_right),
                    (border_radius_br, border_radius.bottom_right),
                    (border_radius_bl, border_radius.bottom_left)
                );
            }

            if let Some(mut border_color) = border_color {
                apply_style!(
                    (border_color_top, border_color.top),
                    (border_color_right, border_color.right),
                    (border_color_bottom, border_color.bottom),
                    (border_color_left, border_color.left)
                );
            }

            if let Some(mut outline) = outline {
                apply_style!(
                    (outline_width, outline.width),
                    (outline_color, outline.color),
                    (outline_offset, outline.offset)
                );
            }

            if let Some(mut text_font) = text_font {
                apply_style!((font_size, text_font.font_size));
            }

            if let Some(mut text_layout) = text_layout {
                apply_style!(
                    (text_justity, text_layout.justify),
                    (text_linebreak, text_layout.linebreak)
                );
            }

            if let Some(mut text_color) = text_color {
                apply_style!((text_color, text_color.0));
            }
        }

        match interaction.into_inner() {
            Interaction::None => {
                apply_style(
                    &picking_styles.base,
                    node,
                    background_color,
                    z_index,
                    border_radius,
                    border_color,
                    outline,
                    text_font,
                    text_layout,
                    text_color,
                );
            }
            Interaction::Hovered => {
                apply_style(
                    &picking_styles.hover,
                    node,
                    background_color,
                    z_index,
                    border_radius,
                    border_color,
                    outline,
                    text_font,
                    text_layout,
                    text_color,
                );
            }
            Interaction::Pressed => {
                apply_style(
                    &picking_styles.focus,
                    node,
                    background_color,
                    z_index,
                    border_radius,
                    border_color,
                    outline,
                    text_font,
                    text_layout,
                    text_color,
                );
            }
        }
    }
}

#[derive(Default, Component)]
#[require(Interaction)]
pub struct PickingStyles {
    pub base: PickingStyle,
    pub hover: PickingStyle,
    pub focus: PickingStyle,
}

#[derive(Default)]
pub struct PickingStyle {
    pub aspect_ratio: Option<Option<f32>>,
    // pub box_sizing: Option<BoxSizing>,
    pub display: Option<Display>,
    pub overflow_x: Option<OverflowAxis>,
    pub overflow_y: Option<OverflowAxis>,
    pub position: Option<PositionType>,
    pub top: Option<Val>,
    pub right: Option<Val>,
    pub bottom: Option<Val>,
    pub left: Option<Val>,
    pub z_index: Option<i32>,
    pub flex_basis: Option<Val>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
    // pub grid_column: Option<GridPlacement>,
    pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
    // pub grid_row: Option<GridPlacement>,
    pub grid_auto_flow: Option<GridAutoFlow>,
    pub grid_auto_columns: Option<Vec<GridTrack>>,
    pub grid_auto_rows: Option<Vec<GridTrack>>,
    pub column_gap: Option<Val>,
    pub row_gap: Option<Val>,
    pub justify_content: Option<JustifyContent>,
    pub justify_items: Option<JustifyItems>,
    pub justify_self: Option<JustifySelf>,
    pub align_content: Option<AlignContent>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub padding_top: Option<Val>,
    pub padding_right: Option<Val>,
    pub padding_bottom: Option<Val>,
    pub padding_left: Option<Val>,
    pub margin_top: Option<Val>,
    pub margin_right: Option<Val>,
    pub margin_bottom: Option<Val>,
    pub margin_left: Option<Val>,
    pub width: Option<Val>,
    pub min_width: Option<Val>,
    pub max_width: Option<Val>,
    pub height: Option<Val>,
    pub min_height: Option<Val>,
    pub max_height: Option<Val>,
    pub font_size: Option<f32>,
    pub text_justity: Option<Justify>,
    pub text_color: Option<Color>,
    pub text_linebreak: Option<LineBreak>,
    pub background_color: Option<Color>,
    pub border_radius_tl: Option<Val>,
    pub border_radius_tr: Option<Val>,
    pub border_radius_br: Option<Val>,
    pub border_radius_bl: Option<Val>,
    pub border_top: Option<Val>,
    pub border_right: Option<Val>,
    pub border_bottom: Option<Val>,
    pub border_left: Option<Val>,
    pub border_color_top: Option<Color>,
    pub border_color_right: Option<Color>,
    pub border_color_bottom: Option<Color>,
    pub border_color_left: Option<Color>,
    pub outline_width: Option<Val>,
    pub outline_color: Option<Color>,
    pub outline_offset: Option<Val>,
}
