use crate::{
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style, val::parse_fraction},
    ParseClassError, ParseCtx, ParseResult,
};

use super::NodeProp;
use quote::quote;

pub fn parse_aspect_ratio(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "aspect" {
        insert_computed_style!(ctx, node, AspectRatio, NodeProp::AspectRatio, 0);
        return Ok(true);
    }

    if !class.starts_with("aspect-") {
        return Ok(false);
    }

    let class = &class["aspect-".len()..];
    let token_stream = match class {
        "auto" => quote! { None },
        "square" => quote! { Some(1.0) },
        "video" => quote! { Some(16.0 / 9.0) },
        _ => {
            let Some(ratio) = parse_fraction(class, true) else {
                return Err(ParseClassError::Unknown);
            };

            quote! { Some(#ratio)}
        }
    };
    deny_computed_style!(ctx);
    insert_picking_style!(ctx, AspectRatio, token_stream.clone());
    ctx.insert_node_prop(NodeProp::AspectRatio, token_stream);

    Ok(true)
}
