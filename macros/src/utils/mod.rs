pub mod color;
pub mod quote;
pub mod val;

pub fn parse_neg(str: &str) -> (bool, &str) {
    let neg = str.starts_with('-');
    let str = if neg { &str[1..] } else { str };
    (neg, str)
}

macro_rules! insert_computed_style {
    ($ctx:expr, $style:ident, $val:expr) => {
        $ctx.components
            .$style
            .insert("0", $val, &$ctx.class_type, 0);
    };
}
