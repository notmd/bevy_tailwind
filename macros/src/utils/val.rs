use proc_macro2::TokenStream;
use quote::quote;

use super::quote::ToTokenStream;

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

impl ToTokenStream for Val {
    fn to_token_stream(&self) -> TokenStream {
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

impl Val {
    pub fn parse(str: &str, settings: ParseValSettings) -> Option<Self> {
        if str.is_empty() {
            return None;
        }

        if str.starts_with("[") && str.ends_with("]") {
            let str = &str[1..str.len() - 1];
            return parse_arbitrary(str, settings);
        }

        match str {
            "px" => {
                if settings.allow_px {
                    return Some(Val::Px(1.0));
                } else {
                    return None;
                }
            }
            "auto" => {
                if settings.allow_auto {
                    return Some(Val::Auto);
                } else {
                    return None;
                }
            }
            "full" => {
                if settings.allow_full {
                    return Some(Val::Percent(100.0));
                } else {
                    return None;
                }
            }
            "screen" => {
                if settings.allow_screen_width {
                    return Some(Val::Vw(100.0));
                } else if settings.allow_screen_height {
                    return Some(Val::Vh(100.0));
                } else {
                    return None;
                }
            }
            "svw" => {
                if settings.allow_dimension_screen_width {
                    return Some(Val::VMin(100.0));
                } else {
                    return None;
                }
            }
            "svh" => {
                if settings.allow_dimension_screen_height {
                    return Some(Val::VMin(100.0));
                } else {
                    return None;
                }
            }
            "lvw" => {
                if settings.allow_dimension_screen_width {
                    return Some(Val::VMax(100.0));
                } else {
                    return None;
                }
            }
            "lvh" => {
                if settings.allow_dimension_screen_height {
                    return Some(Val::VMax(100.0));
                } else {
                    return None;
                }
            }

            _ => {}
        }

        if str.contains("/") {
            if !settings.allow_fraction {
                return None;
            }

            let (num, den) = str.split_once("/")?;
            let num = num.parse::<u32>().ok()?;
            let den = den.parse::<u32>().ok()?;
            if num >= den || den == 0 {
                return None;
            }

            return Some(Val::Percent(num as f32 / den as f32 * 100.));
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

    pub fn eval_neg(self, neg: bool) -> Option<Self> {
        if neg {
            match self {
                Val::Px(val) => Some(Val::Px(-val)),
                Val::VMin(val) => Some(Val::VMin(-val)),
                Val::VMax(val) => Some(Val::VMax(-val)),
                _ => None,
            }
        } else {
            Some(self)
        }
    }
}

pub fn parse_px(str: &str) -> Option<f32> {
    if str.ends_with("px") {
        let str = &str[..str.len() - 2];
        let val = str.parse::<u32>().ok()?;

        return Some(val as f32);
    }

    None
}

pub fn parse_percent(str: &str) -> Option<f32> {
    if str.ends_with("%") {
        let str = &str[..str.len() - 1];
        let val = str.parse::<f32>().ok()?;

        if val < 0.0 || val > 100.0 {
            return None;
        }

        return Some(val);
    }

    None
}

fn parse_arbitrary(str: &str, _settings: ParseValSettings) -> Option<Val> {
    if str.is_empty() {
        return None;
    }

    if let Some(val) = parse_px(str) {
        return Some(Val::Px(val));
    }

    None
}

#[derive(Clone, Copy)]
pub struct ParseValSettings {
    allow_fraction: bool,
    allow_full: bool,
    allow_auto: bool,
    allow_px: bool,
    allow_screen_width: bool,
    allow_screen_height: bool,
    allow_dimension_screen_width: bool,
    allow_dimension_screen_height: bool,
}

impl ParseValSettings {
    pub const fn default_allow() -> Self {
        Self {
            allow_fraction: true,
            allow_full: true,
            allow_auto: true,
            allow_px: true,
            allow_screen_width: true,
            allow_screen_height: true,
            allow_dimension_screen_width: true,
            allow_dimension_screen_height: true,
        }
    }

    pub const fn default_disallow() -> Self {
        Self {
            allow_fraction: false,
            allow_full: false,
            allow_auto: false,
            allow_px: false,
            allow_screen_width: false,
            allow_screen_height: false,
            allow_dimension_screen_width: false,
            allow_dimension_screen_height: false,
        }
    }

    pub const fn allow_fraction(mut self, val: bool) -> Self {
        self.allow_fraction = val;
        self
    }

    pub const fn allow_full(mut self, val: bool) -> Self {
        self.allow_full = val;
        self
    }

    pub const fn allow_auto(mut self, val: bool) -> Self {
        self.allow_auto = val;
        self
    }

    pub const fn allow_px(mut self, val: bool) -> Self {
        self.allow_px = val;
        self
    }

    pub const fn allow_screen_width(mut self, val: bool) -> Self {
        self.allow_screen_width = val;
        self
    }

    pub const fn allow_screen_height(mut self, val: bool) -> Self {
        self.allow_screen_height = val;
        self
    }

    pub const fn allow_dimension_screen_width(mut self, val: bool) -> Self {
        self.allow_dimension_screen_width = val;
        self
    }

    pub const fn allow_dimension_screen_height(mut self, val: bool) -> Self {
        self.allow_dimension_screen_height = val;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(
            Val::parse("1", ParseValSettings::default_allow()),
            Some(Val::Px(4.0))
        );
        assert_eq!(
            Val::parse("12", ParseValSettings::default_allow()),
            Some(Val::Px(48.0))
        );
    }

    #[test]
    fn parse_str() {
        assert_eq!(
            Val::parse("px", ParseValSettings::default_allow()),
            Some(Val::Px(1.0))
        );
        assert_eq!(
            Val::parse("auto", ParseValSettings::default_allow()),
            Some(Val::Auto)
        );
        assert_eq!(
            Val::parse("full", ParseValSettings::default_allow()),
            Some(Val::Percent(100.0))
        );
        assert_eq!(
            Val::parse("1/2", ParseValSettings::default_allow()),
            Some(Val::Percent(50.0))
        );
        assert_eq!(
            Val::parse("1/3", ParseValSettings::default_allow()),
            Some(Val::Percent(1. / 3. * 100.))
        );
        assert_eq!(
            Val::parse("2/3", ParseValSettings::default_allow()),
            Some(Val::Percent(2. / 3. * 100.))
        );
        assert_eq!(
            Val::parse("1/4", ParseValSettings::default_allow()),
            Some(Val::Percent(1. / 4. * 100.))
        );
        assert_eq!(
            Val::parse("2/4", ParseValSettings::default_allow()),
            Some(Val::Percent(1. / 2. * 100.))
        );
        assert_eq!(
            Val::parse("3/4", ParseValSettings::default_allow()),
            Some(Val::Percent(3. / 4. * 100.))
        );
    }

    #[test]
    fn parse_float() {
        assert_eq!(
            Val::parse("1.25", ParseValSettings::default_allow()),
            Some(Val::Px(5.0))
        );
        assert_eq!(
            Val::parse("1.5", ParseValSettings::default_allow()),
            Some(Val::Px(6.0))
        );
        assert_eq!(
            Val::parse("1.75", ParseValSettings::default_allow()),
            Some(Val::Px(7.0))
        );
        assert_eq!(Val::parse("1.50", ParseValSettings::default_allow()), None);
    }

    #[test]
    fn parse_arbitrary() {
        let settings = ParseValSettings::default_allow();

        assert_eq!(Val::parse("[1px]", settings), Some(Val::Px(1.0)));
    }
}
