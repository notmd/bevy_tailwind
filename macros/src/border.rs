use crate::{
    picking::{deny_picking_style, insert_picking_style},
    utils::{color::Color, deny_computed_style, insert_computed_style, val::Val},
    ParseClassError, ParseCtx, ParseResult,
};

impl ParseCtx {
    pub fn parse_border_radius(&mut self, class: &str) -> ParseResult {
        match class {
            "rounded" => {
                insert_computed_style!(
                    self,
                    border_radius,
                    [
                        (BorderRadiusTl, "top_left", 0),
                        (BorderRadiusTr, "top_right", 0),
                        (BorderRadiusBr, "bottom_right", 0),
                        (BorderRadiusBl, "bottom_left", 0)
                    ]
                )
            }
            "rounded-t" => {
                insert_computed_style!(
                    self,
                    border_radius,
                    [
                        (BorderRadiusTl, "top_left", 1),
                        (BorderRadiusTr, "top_right", 1)
                    ]
                )
            }
            "rounded-r" => {
                insert_computed_style!(
                    self,
                    border_radius,
                    [
                        (BorderRadiusTr, "top_right", 1),
                        (BorderRadiusBr, "bottom_right", 1)
                    ]
                )
            }
            "rounded-b" => {
                insert_computed_style!(
                    self,
                    border_radius,
                    [
                        (BorderRadiusBr, "bottom_right", 1),
                        (BorderRadiusBl, "bottom_left", 1)
                    ]
                )
            }
            "rounded-l" => {
                insert_computed_style!(
                    self,
                    border_radius,
                    [
                        (BorderRadiusBl, "bottom_left", 1),
                        (BorderRadiusTl, "top_left", 1)
                    ]
                )
            }
            "rounded-tl" => {
                insert_computed_style!(self, border_radius, BorderRadiusTl, "top_left", 2)
            }
            "rounded-tr" => {
                insert_computed_style!(self, border_radius, BorderRadiusTr, "top_right", 2)
            }
            "rounded-br" => {
                insert_computed_style!(self, border_radius, BorderRadiusBr, "bottom_right", 2)
            }
            "rounded-bl" => {
                insert_computed_style!(self, border_radius, BorderRadiusBl, "bottom_left", 2)
            }
            _ => {}
        }
        if !class.starts_with("rounded") {
            return Ok(false);
        }

        let class = &class["rounded".len()..];

        macro_rules! insert_props {
            ($ctx:ident, $value:expr, $priority:literal, $props:expr) => {
                for prop in $props {
                    $ctx.components
                        .border_radius
                        .insert(prop, $value, &$ctx.class_type, $priority);
                }

                return Ok(true);
            };
        }

        if let Ok(size) = parse_size(class) {
            // rounded*
            deny_computed_style!(self);
            deny_picking_style!(self);
            insert_props!(
                self,
                size,
                0,
                ["top_left", "top_right", "bottom_left", "bottom_right"]
            );
        }

        let class = &class[1..];

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
            insert_props!(self, parse_size(class)?, 2, ["top_right"]);
        }

        if class.starts_with("br") {
            let class = &class["br".len()..];
            let size = parse_size(class)?;
            deny_computed_style!(self);
            insert_picking_style!(self, BorderRadiusBr, size);
            insert_props!(self, parse_size(class)?, 2, ["bottom_right"]);
        }

        if class.starts_with("bl") {
            let class = &class["bl".len()..];
            let size = parse_size(class)?;
            deny_computed_style!(self);
            insert_picking_style!(self, BorderRadiusBl, size);
            insert_props!(self, parse_size(class)?, 2, ["bottom_left"]);
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
                deny_picking_style!(self);
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
