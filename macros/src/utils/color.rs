use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use super::quote::ToTokenStream;

#[derive(Clone)]
pub struct Color {
    name: String,
    level: Option<String>,
    alpha: Option<f32>,
}

impl ToTokenStream for Color {
    fn to_token_stream(&self) -> TokenStream {
        let color = match &self.level {
            Some(level) => {
                let color = format_ident!("{}_{}", self.name.to_uppercase(), level);

                quote! {
                    bevy::color::Color::Srgba(
                       bevy::color::palettes::tailwind::#color
                    )
                }
            }
            None => {
                let color = match self.name.as_str() {
                    "transparent" => quote! {NONE},
                    "black" => quote! {BLACK},
                    "white" => quote! {WHITE},
                    _ => unreachable!("Invalid color name: {}", self.name),
                };

                quote! {
                    bevy::color::Color::#color
                }
            }
        };

        if let Some(alpha) = self.alpha {
            quote! {
                <bevy::color::Color as bevy::color::Alpha>::with_alpha(&#color, #alpha)
            }
        } else {
            color
        }
    }
}

impl Color {
    pub fn parse(str: &str) -> Option<Self> {
        let (name, level) = match str.split_once('-') {
            Some(val) => val,
            None => match str.split_once("/") {
                Some((name, alpha)) => {
                    if !is_valid_simple_name(name) {
                        return None;
                    }
                    let alpha = parse_alpha(alpha)?;

                    return Some(Color {
                        name: name.to_string(),
                        level: None,
                        alpha: Some(alpha),
                    });
                }
                _ => {
                    if !is_valid_simple_name(str) {
                        return None;
                    }
                    return Some(Color {
                        name: str.to_string(),
                        level: None,
                        alpha: None,
                    });
                }
            },
        };

        let (level, alpha) = if let Some((level, alpha)) = level.split_once("/") {
            let alpha = parse_alpha(alpha)?;
            (level, Some(alpha))
        } else {
            (level, None)
        };

        if !is_valid_name(name) || !is_valid_level(level) {
            return None;
        }

        // quote(name, Some(level), alpha)
        Some(Color {
            name: name.to_string(),
            level: Some(level.to_string()),
            alpha,
        })
    }
}

fn parse_alpha(str: &str) -> Option<f32> {
    let alpha = str.parse::<u8>().ok()?;
    if alpha > 100 {
        None
    } else {
        Some(alpha as f32 / 100.0)
    }
}

fn is_valid_name(name: &str) -> bool {
    matches!(
        name,
        "slate"
            | "gray"
            | "zinc"
            | "neutral"
            | "stone"
            | "red"
            | "orange"
            | "amber"
            | "yellow"
            | "lime"
            | "green"
            | "emerald"
            | "teal"
            | "cyan"
            | "sky"
            | "blue"
            | "indigo"
            | "violet"
            | "purple"
            | "fuchsia"
            | "pink"
            | "rose"
    )
}

fn is_valid_level(level: &str) -> bool {
    matches!(
        level,
        "50" | "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900" | "950"
    )
}

fn is_valid_simple_name(name: &str) -> bool {
    matches!(name, "transparent" | "black" | "white")
}
