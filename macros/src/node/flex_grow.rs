use crate::{ParseCtx, ParseResult};
use quote::quote;

use super::NodeProp;

pub fn parse_flex_grow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "flex-grow" | "grow" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                1.0
            });
            Ok(true)
        }
        "flex-grow-0" | "grow-0" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                0.0
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
