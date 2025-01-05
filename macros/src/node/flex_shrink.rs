use quote::quote;

use crate::{ParseCtx, ParseResult};

use super::NodeProp;

pub fn parse_flex_shrink(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "flex-shrink" | "shrink" => {
            ctx.insert_node_prop(NodeProp::FlexShrink, quote! {
                1.0
            });
            Ok(true)
        }
        "flex-shrink-0" | "shrink-0" => {
            ctx.insert_node_prop(NodeProp::FlexShrink, quote! {
                0.0
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
