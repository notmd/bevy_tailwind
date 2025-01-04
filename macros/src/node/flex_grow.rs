use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_flex_grow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "grow" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                1.0
            });
            Ok(true)
        }
        "grow-0" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                0.0
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
