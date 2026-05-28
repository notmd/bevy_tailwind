use crate::{
    ClassType, ParseClassError, ParseCtx, ParseResult,
    picking::{PickingStyleProp, deny_picking_style, insert_picking_style},
    utils::{color::Color, deny_computed_style, insert_computed_style, val::Val},
};

impl ParseCtx {
    pub fn parse_border_radius(&mut self, class: &str) -> ParseResult {
        macro_rules! insert_props {
            ($ctx:ident, $value:expr, $priority:literal, $props:expr) => {
                crate::node::insert_node_prop_nested!(
                    $ctx,
                    crate::node::NodeProp::BorderRadius,
                    quote::quote! {bevy::ui::BorderRadius},
                    $value,
                    $priority,
                    $props
                );
            };
        }

        macro_rules! handle_computed {
            ($ctx:ident, [$($picking_prop:ident),*], $priority:literal, $props:expr) => {
                if let ClassType::Computed(expr) = $ctx.class_type.clone() {
                    if $ctx.hover || $ctx.focus {
                        $(
                            $ctx.insert_picking_style(PickingStyleProp::$picking_prop, expr.clone());
                        )*
                        return Ok(true);
                    } else {
                        insert_props!($ctx, expr.clone(), $priority, $props);
                    }
                }
            };
            ($ctx:ident, $picking_prop:ident, $priority:literal, $props:expr) => {
                if let ClassType::Computed(expr) = $ctx.class_type.clone() {
                    if $ctx.hover || $ctx.focus {
                        $ctx.insert_picking_style(PickingStyleProp::$picking_prop, expr.clone());
                        return Ok(true);
                    } else {
                        insert_props!($ctx, expr.clone(), $priority, $props);
                    }
                }
            };
        }

        match class {
            "rounded" => {
                handle_computed!(
                    self,
                    [
                        BorderRadiusTl,
                        BorderRadiusTr,
                        BorderRadiusBr,
                        BorderRadiusBl
                    ],
                    0,
                    ["top_left", "top_right", "bottom_right", "bottom_left"]
                );
            }
            "rounded-t" => {
                handle_computed!(
                    self,
                    [BorderRadiusTl, BorderRadiusTr],
                    1,
                    ["top_left", "top_right"]
                );
            }
            "rounded-r" => {
                handle_computed!(
                    self,
                    [BorderRadiusTr, BorderRadiusBr],
                    1,
                    ["top_right", "bottom_right"]
                );
            }
            "rounded-b" => {
                handle_computed!(
                    self,
                    [BorderRadiusBr, BorderRadiusBl],
                    1,
                    ["bottom_right", "bottom_left"]
                );
            }
            "rounded-l" => {
                handle_computed!(
                    self,
                    [BorderRadiusBl, BorderRadiusTl],
                    1,
                    ["bottom_left", "top_left"]
                );
            }
            "rounded-tl" => {
                handle_computed!(self, BorderRadiusTl, 2, ["top_left"]);
            }
            "rounded-tr" => {
                handle_computed!(self, BorderRadiusTr, 2, ["top_right"]);
            }
            "rounded-br" => {
                handle_computed!(self, BorderRadiusBr, 2, ["bottom_right"]);
            }
            "rounded-bl" => {
                handle_computed!(self, BorderRadiusBl, 2, ["bottom_left"]);
            }
            _ => {}
        }

        if !class.starts_with("rounded") {
            return Ok(false);
        }

        let class = &class["rounded".len()..];

        if let Ok(size) = parse_size(class) {
            deny_computed_style!(self);
            deny_picking_style!(self);
            insert_props!(
                self,
                size,
                0,
                ["top_left", "top_right", "bottom_left", "bottom_right"]
            );
        }

        let class = if class.is_empty() { class } else { &class[1..] };

        if class.starts_with("tl") {
            let class = &class["tl".len()..];
            let size = parse_size(class)?;
            deny_computed_style!(self);
            insert_picking_style!(self, BorderRadiusTl, size);
            insert_props!(self, size, 2, ["top_left"]);
        }

        if class.starts_with("tr") {
            let class = &class["tr".len()..];
            let size = parse_size(class)?;
            deny_computed_style!(self);
            insert_picking_style!(self, BorderRadiusTr, size);
            insert_props!(self, size, 2, ["top_right"]);
        }

        if class.starts_with("br") {
            let class = &class["br".len()..];
            let size = parse_size(class)?;
            deny_computed_style!(self);
            insert_picking_style!(self, BorderRadiusBr, size);
            insert_props!(self, size, 2, ["bottom_right"]);
        }

        if class.starts_with("bl") {
            let class = &class["bl".len()..];
            let size = parse_size(class)?;
            deny_computed_style!(self);
            insert_picking_style!(self, BorderRadiusBl, size);
            insert_props!(self, size, 2, ["bottom_left"]);
        }

        if class.starts_with("t") {
            let class = &class["t".len()..];
            deny_computed_style!(self);
            deny_picking_style!(self);
            insert_props!(self, parse_size(class)?, 1, ["top_left", "top_right"]);
        }

        if class.starts_with("r") {
            let class = &class["r".len()..];
            deny_computed_style!(self);
            deny_picking_style!(self);
            insert_props!(self, parse_size(class)?, 1, ["top_right", "bottom_right"]);
        }

        if class.starts_with("b") {
            let class = &class["b".len()..];
            deny_computed_style!(self);
            deny_picking_style!(self);
            insert_props!(self, parse_size(class)?, 1, ["bottom_right", "bottom_left"]);
        }

        if class.starts_with("l") {
            let class = &class["l".len()..];
            deny_computed_style!(self);
            deny_picking_style!(self);
            insert_props!(self, parse_size(class)?, 1, ["bottom_left", "top_left"]);
        }

        Ok(false)
    }

    pub fn parse_border_color(&mut self, class: &str) -> ParseResult {
        match class {
            "border-color" => {
                insert_computed_style!(
                    self,
                    border_color,
                    [
                        (BorderColorTop, "top", 0),
                        (BorderColorRight, "right", 0),
                        (BorderColorBottom, "bottom", 0),
                        (BorderColorLeft, "left", 0)
                    ]
                );
            }
            "border-x-color" => {
                insert_computed_style!(
                    self,
                    border_color,
                    [(BorderColorRight, "right", 1), (BorderColorLeft, "left", 1)]
                );
            }
            "border-y-color" => {
                insert_computed_style!(
                    self,
                    border_color,
                    [(BorderColorTop, "top", 1), (BorderColorBottom, "bottom", 1)]
                );
            }
            "border-t-color" => {
                insert_computed_style!(self, border_color, [(BorderColorTop, "top", 1)]);
            }
            "border-r-color" => {
                insert_computed_style!(self, border_color, [(BorderColorRight, "right", 0)]);
            }
            "border-b-color" => {
                insert_computed_style!(self, border_color, [(BorderColorBottom, "bottom", 0)]);
            }
            "border-l-color" => {
                insert_computed_style!(self, border_color, [(BorderColorLeft, "left", 0)]);
            }
            _ => {}
        };

        if !class.starts_with("border-") {
            return Ok(false);
        }

        macro_rules! insert_props {
            ($ctx:ident, $value:expr, $priority:literal, $props:expr) => {
                for prop in $props {
                    $ctx.components.border_color.insert(
                        prop,
                        $value.clone(),
                        &$ctx.class_type,
                        $priority,
                    );
                }

                return Ok(true);
            };
        }

        let class = &class["border-".len()..];

        if class.starts_with("x-") {
            let class = &class["x-".len()..];
            if let Some(color) = Color::parse(class) {
                deny_computed_style!(self);
                insert_props!(self, color, 1, ["right", "left"]);
            }
            return Ok(false);
        }

        if class.starts_with("y-") {
            let class = &class["y-".len()..];
            if let Some(color) = Color::parse(class) {
                deny_computed_style!(self);
                insert_props!(self, color, 1, ["top", "bottom"]);
            }
            return Ok(false);
        }

        if class.starts_with("t-") {
            let class = &class["t-".len()..];
            if let Some(color) = Color::parse(class) {
                deny_computed_style!(self);
                insert_props!(self, color, 2, ["top"]);
            }
            return Ok(false);
        }

        if class.starts_with("r-") {
            let class = &class["r-".len()..];
            if let Some(color) = Color::parse(class) {
                deny_computed_style!(self);
                insert_props!(self, color, 2, ["right"]);
            }
            return Ok(false);
        }

        if class.starts_with("b-") {
            let class = &class["b-".len()..];
            if let Some(color) = Color::parse(class) {
                deny_computed_style!(self);
                insert_props!(self, color, 2, ["bottom"]);
            }
            return Ok(false);
        }

        if class.starts_with("l-") {
            let class = &class["l-".len()..];
            if let Some(color) = Color::parse(class) {
                deny_computed_style!(self);
                insert_props!(self, color, 2, ["left"]);
            }
            return Ok(false);
        }

        let Some(color) = Color::parse(class) else {
            return Ok(false);
        };

        deny_computed_style!(self);
        insert_props!(self, color, 0, ["top", "right", "bottom", "left"]);
    }
}

fn parse_size(class: &str) -> Result<Val, ParseClassError> {
    let class = if class.is_empty() { class } else { &class[1..] };

    let px = match class {
        "none" => 0.,
        "sm" => 2.,
        "" => 4.,
        "md" => 6.,
        "lg" => 8.,
        "xl" => 12.,
        "2xl" => 16.,
        "3xl" => 24.,
        "full" => 9999.,
        _ => return Err(ParseClassError::Unknown),
    };

    Ok(Val::Px(px))
}
