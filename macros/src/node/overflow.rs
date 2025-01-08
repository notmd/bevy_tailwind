use crate::{
    ParseClassError, ParseCtx, ParseResult,
    utils::{StructPropValue, ToTokenStream},
};
use quote::quote;

use super::NodeProp;

pub fn parse_overflow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("overflow-") {
        return Ok(false);
    }

    let class = &class["overflow-".len()..];

    if let Ok(axis) = parse_axis(class) {
        let overflow = ctx
            .components
            .node
            .entry(NodeProp::Overflow)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, Overflow::default()))
            .value
            .downcast_mut::<Overflow>();

        if overflow.x.is_none() {
            overflow.x = Some(axis);
        }

        if overflow.y.is_none() {
            overflow.y = Some(axis);
        }

        return Ok(true);
    }

    if class.starts_with("x-") {
        let class = &class["x-".len()..];
        let axis = parse_axis(class)?;
        let overflow = ctx
            .components
            .node
            .entry(NodeProp::Overflow)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, Overflow::default()))
            .value
            .downcast_mut::<Overflow>();
        overflow.x = Some(axis);
        return Ok(true);
    }

    if class.starts_with("y-") {
        let class = &class["y-".len()..];
        let axis = parse_axis(class)?;
        let overflow = ctx
            .components
            .node
            .entry(NodeProp::Overflow)
            .or_insert_with(|| StructPropValue::nested(ctx.class_type, Overflow::default()))
            .value
            .downcast_mut::<Overflow>();
        overflow.y = Some(axis);
        return Ok(true);
    }

    Ok(false)
}

fn parse_axis(class: &str) -> Result<OverflowAxis, ParseClassError> {
    let axis = match class {
        "visible" => OverflowAxis::Visible,
        "clip" => OverflowAxis::Clip,
        "hidden" => OverflowAxis::Hidden,
        "scroll" => OverflowAxis::Scroll,
        _ => return Err(ParseClassError::Unknown),
    };

    Ok(axis)
}

#[derive(Clone, Copy)]
enum OverflowAxis {
    Visible,
    Clip,
    Hidden,
    Scroll,
}

impl ToTokenStream for OverflowAxis {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            OverflowAxis::Visible => quote! { bevy::ui::OverflowAxis::Visible },
            OverflowAxis::Clip => quote! { bevy::ui::OverflowAxis::Clip },
            OverflowAxis::Hidden => quote! { bevy::ui::OverflowAxis::Hidden },
            OverflowAxis::Scroll => quote! { bevy::ui::OverflowAxis::Scroll },
        }
    }
}

#[derive(Default)]
struct Overflow {
    x: Option<OverflowAxis>,
    y: Option<OverflowAxis>,
}

impl ToTokenStream for Overflow {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let x = self.x.map(|v| {
            let v = v.to_token_stream();
            quote! {
                x: #v
            }
        });
        let y = self.y.map(|v| {
            let v = v.to_token_stream();
            quote! {
                y: #v
            }
        });

        let props = [x, y].into_iter().filter(Option::is_some);

        quote! {
            bevy::ui::Overflow {
                #(#props,)*
                ..Default::default()
            }
        }
    }

    fn structual_to_token_stream(&self) -> Option<crate::utils::StructualTokenStream> {
        let mut res = crate::utils::StructualTokenStream::default();
        if let Some(ref x) = self.x {
            res.push(("x", x.to_token_stream()));
        }

        if let Some(ref y) = self.y {
            res.push(("y", y.to_token_stream()));
        }

        Some(res)
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}
