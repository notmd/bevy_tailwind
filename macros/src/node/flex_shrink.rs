use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_shrink(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "shrink" => {
            ctx.node_props.insert(NodeProp::FlexShrink, quote! {
                1.0
            });
            Ok(true)
        }
        "shrink-0" => {
            ctx.node_props.insert(NodeProp::FlexShrink, quote! {
                0.0
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
