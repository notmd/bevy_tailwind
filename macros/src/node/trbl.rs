use crate::{
    ParseCtx, ParseResult,
    utils::{IntoTokenStream, val::parse_val},
};

use super::NodeProp;
use quote::quote;

pub fn parse_trbl(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let Some((prefix, val)) = class.split_once("-") else {
        return Ok(false);
    };

    let Some(val) = parse_val(val).map(|val| val.into_token_stream()) else {
        return Ok(false);
    };

    match prefix {
        "top" => {
            ctx.insert_node_prop(NodeProp::Top, quote! {
                #val
            });
            Ok(true)
        }
        "right" => {
            ctx.insert_node_prop(NodeProp::Right, quote! {
                #val
            });
            Ok(true)
        }
        "bottom" => {
            ctx.insert_node_prop(NodeProp::Bottom, quote! {
                #val
            });
            Ok(true)
        }
        "left" => {
            ctx.insert_node_prop(NodeProp::Left, quote! {
                #val
            });
            Ok(true)
        }
        _ => Ok(false),
    }
}
