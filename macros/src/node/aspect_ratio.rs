use crate::{picking::insert_picking_style, ParseClassError, ParseCtx, ParseResult};

use super::NodeProp;
use quote::quote;

pub fn parse_aspect_ratio(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("aspect-") {
        return Ok(false);
    }

    let class = &class["aspect-".len()..];
    let token_stream = match class {
        "auto" => quote! { None },
        "square" => quote! { Some(1.0) },
        "video" => quote! { Some(16.0 / 9.0) },
        _ => return Err(ParseClassError::Unknown),
    };
    insert_picking_style!(ctx, AspectRatio, token_stream.clone());
    ctx.insert_node_prop(NodeProp::AspectRatio, token_stream);

    Ok(true)
}
