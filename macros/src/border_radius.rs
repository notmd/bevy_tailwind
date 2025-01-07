use proc_macro2::TokenStream;

use crate::{
    ParseCtx, ParseResult,
    utils::{StructPropValue, ToTokenStream, val::Val},
};

impl ParseCtx {
    pub fn parse_border_radius(&mut self, class: &str) -> ParseResult {
        if !class.starts_with("rounded") {
            return Ok(false);
        }

        let class = &class["rounded".len()..];

        macro_rules! insert_props {
            ($ctx:ident, $value:expr, $priority:literal , [$($prop:literal),*]) => {
                $(
                    let value = $ctx.components.border_radius.get_mut($prop);

                    if let Some(value) = value {
                        let value = value.value.downcast_mut::<BorderRadiusVal>();

                        if value.priority <= $priority {
                            value.priority = $priority;
                        }
                    } else {
                        $ctx.components.border_radius.insert(
                            $prop,
                            StructPropValue::wrapped($ctx.class_type, BorderRadiusVal {
                                val: $value,
                                priority: $priority,
                            }),
                        );
                    }
                )*
            };
        }

        let class = if class.is_empty() { class } else { &class[1..] };

        if let Some(size) = parse_size_otp(class) {
            insert_props!(self, size, 0, [
                "top_left",
                "top_right",
                "bottom_left",
                "bottom_right"
            ]);

            return Ok(true);
        }

        if class.starts_with("tl") {
            let class = &class["tl".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 2, ["top_left"]);

            return Ok(true);
        }

        if class.starts_with("tr") {
            let class = &class["tr".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 2, ["top_right"]);

            return Ok(true);
        }

        if class.starts_with("br") {
            let class = &class["br".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 2, ["bottom_right"]);

            return Ok(true);
        }

        if class.starts_with("bl") {
            let class = &class["bl".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 2, ["bottom_left"]);

            return Ok(true);
        }

        if class.starts_with("t") {
            let class = &class["t".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 1, ["top_left", "top_right"]);

            return Ok(true);
        }

        if class.starts_with("r") {
            let class = &class["r".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 1, ["top_right", "bottom_right"]);

            return Ok(true);
        }

        if class.starts_with("b") {
            let class = &class["b".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 1, ["bottom_right", "bottom_left"]);

            return Ok(true);
        }

        if class.starts_with("l") {
            let class = &class["l".len()..];
            let class = if class.is_empty() { class } else { &class[1..] };

            insert_props!(self, parse_size(class), 1, ["bottom_left", "top_left"]);

            return Ok(true);
        }

        Ok(false)
    }
}

fn parse_size(class: &str) -> Val {
    parse_size_otp(class).expect("Invalid border radius size ")
}

fn parse_size_otp(class: &str) -> Option<Val> {
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
        _ => return None,
    };

    Some(Val::Px(px))
}

struct BorderRadiusVal {
    val: Val,
    priority: u8,
}

impl ToTokenStream for BorderRadiusVal {
    fn to_token_stream(&self) -> TokenStream {
        self.val.to_token_stream()
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}
