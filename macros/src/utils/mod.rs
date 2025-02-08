pub mod color;
pub mod quote;
pub mod val;

pub fn parse_neg(str: &str) -> (bool, &str) {
    let neg = str.starts_with('-');
    let str = if neg { &str[1..] } else { str };
    (neg, str)
}

macro_rules! insert_computed_style {
    ($ctx:ident, $component:ident, $picking_prop:ident, $component_prop:expr, $priority:literal) => {
        match $ctx.class_type.clone() {
            crate::ClassType::Computed(expr) => {
                crate::picking::insert_picking_style!($ctx, $picking_prop, expr);
                $ctx.components.$component.insert(
                    $component_prop,
                    expr,
                    &$ctx.class_type,
                    $priority,
                );
                return Ok(true);
            }
            _ => {}
        }
    };

    // multiple
    ($ctx:ident, $component:ident, [$(($picking_prop:ident, $component_prop:expr, $priority:literal)),+]) => {
        match $ctx.class_type.clone() {
            crate::ClassType::Computed(expr) => {
                $(
                    crate::picking::insert_picking_style!($ctx, $picking_prop, expr);
                    $ctx.components.$component.insert(
                        $component_prop,
                        expr.clone(),
                        &$ctx.class_type,
                        $priority,
                    );
                )+
                return Ok(true);
            }
            _ => {}
        }
    };
}

pub(crate) use insert_computed_style;

macro_rules! deny_computed_style {
    ($ctx:ident) => {
        match $ctx.class_type {
            crate::ClassType::Computed(_) => {
                return Err(crate::ParseClassError::Unknown);
            }
            _ => {}
        }
    };
}

pub(crate) use deny_computed_style;
