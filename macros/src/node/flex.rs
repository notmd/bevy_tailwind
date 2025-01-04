use crate::{
    ParseCtx, ParseResult,
    utils::{IntoTokenStream, val::Val},
};
use quote::quote;

use super::NodeProp;

pub fn parse_flex(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "flex-1" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                1.0
            });
            ctx.node_props.insert(NodeProp::FlexShrink, quote! {
                1.0
            });
            ctx.node_props
                .insert(NodeProp::FlexBasis, Val::Percent(0.).into_token_stream());
            Ok(true)
        }
        "flex-auto" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                1.0
            });
            ctx.node_props.insert(NodeProp::FlexShrink, quote! {
                1.0
            });
            ctx.node_props
                .insert(NodeProp::FlexBasis, Val::Auto.into_token_stream());
            Ok(true)
        }
        "flex-initial" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                0.0
            });
            ctx.node_props.insert(NodeProp::FlexShrink, quote! {
                1.0
            });
            ctx.node_props
                .insert(NodeProp::FlexBasis, Val::Auto.into_token_stream());
            Ok(true)
        }
        "flex-none" => {
            ctx.node_props.insert(NodeProp::FlexGrow, quote! {
                0.0
            });
            ctx.node_props.insert(NodeProp::FlexShrink, quote! {
                0.0
            });
            ctx.node_props
                .insert(NodeProp::FlexBasis, Val::Auto.into_token_stream());
            Ok(true)
        }
        _ => Ok(false),
    }
}
