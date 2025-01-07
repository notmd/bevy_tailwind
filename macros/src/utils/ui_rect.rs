use quote::quote;

use super::{PrioritizedStructPropValue, StructualTokenStream, ToTokenStream, val::Val};

#[derive(Default, Clone)]
pub struct UiRect {
    pub top: Option<PrioritizedStructPropValue<Val>>,
    pub right: Option<PrioritizedStructPropValue<Val>>,
    pub bottom: Option<PrioritizedStructPropValue<Val>>,
    pub left: Option<PrioritizedStructPropValue<Val>>,
}

impl ToTokenStream for UiRect {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let top = self.top.map(|v| {
            let v = v.to_token_stream();
            quote! {
                top: #v
            }
        });
        let right = self.right.map(|v| {
            let v = v.to_token_stream();
            quote! {
                right: #v
            }
        });
        let bottom = self.bottom.map(|v| {
            let v = v.to_token_stream();
            quote! {
                bottom: #v
            }
        });
        let left = self.left.map(|v| {
            let v = v.to_token_stream();
            quote! {
                left: #v
            }
        });

        let props = [top, right, bottom, left]
            .into_iter()
            .filter(Option::is_some);

        quote! {
            bevy::ui::UiRect {
                #(#props,)*
               ..Default::default()
            }
        }
    }

    fn structual_to_token_stream(&self) -> Option<StructualTokenStream> {
        let mut res = StructualTokenStream::default();
        if let Some(ref top) = self.top {
            res.push(("top", top.to_token_stream()));
        }

        if let Some(ref right) = self.right {
            res.push(("right", right.to_token_stream()));
        }

        if let Some(ref bottom) = self.bottom {
            res.push(("bottom", bottom.to_token_stream()));
        }

        if let Some(ref left) = self.left {
            res.push(("left", left.to_token_stream()));
        }

        Some(res)
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

macro_rules! insert_node_ui_rect_props {
    ($ctx:ident, $node_prop:expr, $value:expr, $priority:literal , [$($prop:ident),*]) => {
        $(
            let prop = &mut $ctx.components
                .node
                .entry($node_prop)
                .or_insert_with(|| StructPropValue::nested($ctx.class_type, UiRect::default()))
                .value
                .downcast_mut::<UiRect>()
                .$prop;

            if let Some(prop) = prop.as_mut() {
                prop.set_if_gte_priority($value, $priority);
            } else {
                prop.replace(PrioritizedStructPropValue::new($value, $priority));
            }
        )*

        return Ok(true);
    };
}

pub(crate) use insert_node_ui_rect_props;
