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
}

pub(crate) use insert_computed_style;
