use super::NodeProp;
use crate::{ParseClassError, ParseCtx, ParseResult};
use crate::{
    node::insert_node_ui_rect,
    utils::val::Val,
};

pub fn parse_border(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("border") {
        return Ok(false);
    }

    let class = &class["border".len()..];

    if let Ok(val) = parse_val(class) {
        // border*
        insert_node_ui_rect!(ctx, NodeProp::Border, val, 0, [
            "top", "right", "bottom", "left"
        ]);
    }

    let class = if class.is_empty() { class } else { &class[1..] };

    if class.starts_with("x") {
        let class = &class["x".len()..];
        insert_node_ui_rect!(ctx, NodeProp::Border, parse_val(class)?, 1, [
            "left", "right"
        ]);
    }

    if class.starts_with("y") {
        let class = &class["y".len()..];
        insert_node_ui_rect!(ctx, NodeProp::Border, parse_val(class)?, 1, [
            "top", "bottom"
        ]);
    }

    if class.starts_with("t") {
        let class = &class["t".len()..];
        insert_node_ui_rect!(ctx, NodeProp::Border, parse_val(class)?, 1, ["top"]);
    }

    if class.starts_with("r") {
        let class = &class["r".len()..];
        insert_node_ui_rect!(ctx, NodeProp::Border, parse_val(class)?, 1, ["right"]);
    }

    if class.starts_with("b") {
        let class = &class["b".len()..];
        insert_node_ui_rect!(ctx, NodeProp::Border, parse_val(class)?, 1, ["bottom"]);
    }

    if class.starts_with("l") {
        let class = &class["l".len()..];
        insert_node_ui_rect!(ctx, NodeProp::Border, parse_val(class)?, 1, ["left"]);
    }

    Ok(false)
}

fn parse_val(class: &str) -> Result<Val, ParseClassError> {
    let class = if class.is_empty() { class } else { &class[1..] };

    let px = match class {
        "" => 1.,
        _ => {
            let px = class.parse::<u32>().map_err(|_| ParseClassError::Unknown)?;

            px as f32
        }
    };

    Ok(Val::Px(px))
}
