use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use super::IntoTokenStream;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Val {
    Auto,
    Px(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    VMin(f32),
    VMax(f32),
}

impl IntoTokenStream for Val {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Val::Auto => quote! { bevy::ui::Val::Auto },
            Val::Px(val) => quote! { bevy::ui::Val::Px(#val) },
            Val::Percent(val) => quote! { bevy::ui::Val::Percent(#val) },
            Val::Vw(val) => quote! { bevy::ui::Val::Vw(#val) },
            Val::Vh(val) => quote! { bevy::ui::Val::Vh(#val) },
            Val::VMin(val) => quote! { bevy::ui::Val::VMin(#val) },
            Val::VMax(val) => quote! { bevy::ui::Val::VMax(#val) },
        }
    }
}

pub fn parse_val(str: &str) -> Option<Val> {
    if str.is_empty() {
        return None;
    }

    match str {
        "px" => return Some(Val::Px(1.0)),
        "auto" => return Some(Val::Auto),
        "full" => return Some(Val::Percent(100.0)),
        "1/2" => return Some(Val::Percent(50.0)),
        "1/3" => return Some(Val::Percent(1. / 3.)),
        "2/3" => return Some(Val::Percent(2. / 3.)),
        "1/4" => return Some(Val::Percent(1. / 4.)),
        "2/4" => return Some(Val::Percent(1. / 2.)),
        "3/4" => return Some(Val::Percent(3. / 4.)),
        _ => {}
    }

    if let Ok(val) = str.parse::<u32>() {
        return Some(Val::Px(val as f32 * 4.0));
    }

    if let Ok(val) = str.parse::<f32>() {
        let (_, fract) = str.split_once(".")?;
        match fract {
            "25" | "5" | "75" => {
                return Some(Val::Px(val * 4.0));
            }
            _ => return None,
        }
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(parse_val("1"), Some(Val::Px(4.0)));
        assert_eq!(parse_val("12"), Some(Val::Px(48.0)));
    }

    #[test]
    fn parse_str() {
        #[test]
        fn parse_str() {
            assert_eq!(parse_val("px"), Some(Val::Px(1.0)));
            assert_eq!(parse_val("auto"), Some(Val::Auto));
            assert_eq!(parse_val("full"), Some(Val::Percent(100.0)));
            assert_eq!(parse_val("1/2"), Some(Val::Percent(50.0)));
            assert_eq!(parse_val("1/3"), Some(Val::Percent(1. / 3.)));
            assert_eq!(parse_val("2/3"), Some(Val::Percent(2. / 3.)));
            assert_eq!(parse_val("1/4"), Some(Val::Percent(1. / 4.)));
            assert_eq!(parse_val("2/4"), Some(Val::Percent(1. / 2.)));
            assert_eq!(parse_val("3/4"), Some(Val::Percent(3. / 4.)));
        }
    }

    #[test]
    fn parse_float() {
        assert_eq!(parse_val("1.25"), Some(Val::Px(5.0)));
        assert_eq!(parse_val("1.5"), Some(Val::Px(6.0)));
        assert_eq!(parse_val("1.75"), Some(Val::Px(7.0)));
        assert_eq!(parse_val("1.50"), None);
    }
}
