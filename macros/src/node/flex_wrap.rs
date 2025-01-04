use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_wrap(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let token = match class {
        "flex-wrap" => {
            quote! {
                bevy::ui::FlexWrap::Wrap
            }
        }
        "flex-wrap-reverse" => {
            quote! {
                bevy::ui::FlexWrap::WrapReverse
            }
        }
        "flex-nowrap" => {
            quote! {
                bevy::ui::FlexWrap::NoWrap
            }
        }
        _ => return Ok(false),
    };

    ctx.node_props.insert(NodeProp::FlexWrap, token);

    Ok(true)
}
