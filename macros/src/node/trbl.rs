use crate::{ParseCtx, ParseResult, val::parse_val};

use super::NodeProp;
use quote::quote;

pub fn parse_trbl(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let Some((prefix, val)) = class.split_once("-") else {
        return Ok(false);
    };

    let Some(val) = parse_val(val).map(|val| val.to_token_stream()) else {
        return Ok(false);
    };

    match prefix {
        "top" => {
            ctx.node_props.insert(NodeProp::Top, quote! {
                #val
            });
            Ok(true)
        }
        "right" => {
            ctx.node_props.insert(NodeProp::Right, quote! {
                #val
            });
            Ok(true)
        }
        "bottom" => {
            ctx.node_props.insert(NodeProp::Bottom, quote! {
                #val
            });
            Ok(true)
        }
        "left" => {
            ctx.node_props.insert(NodeProp::Left, quote! {
                #val
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
