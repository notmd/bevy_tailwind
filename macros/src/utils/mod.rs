use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::{ClassType, MacroType};
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub mod val;

pub fn parse_neg(str: &str) -> (bool, &str) {
    let neg = str.starts_with('-');
    let str = if neg { &str[1..] } else { str };
    (neg, str)
}

pub trait IntoTokenStream {
    fn into_token_stream(self) -> proc_macro2::TokenStream;
}

pub trait AsStr {
    fn as_str(&self) -> &'static str;
}

pub(crate) struct StructProps<K: AsStr>(pub IndexMap<K, (proc_macro2::TokenStream, ClassType)>);

impl<T: AsStr> Default for StructProps<T> {
    fn default() -> Self {
        Self(IndexMap::new())
    }
}

impl<T: AsStr> Deref for StructProps<T> {
    type Target = IndexMap<T, (proc_macro2::TokenStream, ClassType)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: AsStr> DerefMut for StructProps<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: AsStr + Hash + Eq> StructProps<T> {
    pub fn insert(&mut self, key: T, value: proc_macro2::TokenStream, class_type: ClassType) {
        self.0.insert(key, (value, class_type));
    }

    pub fn quote(
        &self,
        condition_idents: &[TokenStream],
        macro_type: &MacroType,
    ) -> Option<TokenStream> {
        if self.0.is_empty() {
            return None;
        }

        let sep = macro_type.sep_token();
        let end = macro_type.end_token();
        let props: Vec<TokenStream> = self
            .0
            .iter()
            .map(|(prop, (value, class_type))| {
                let prop = Ident::new(prop.as_str(), Span::call_site());

                match class_type {
                    ClassType::String => {
                        quote! {
                            #prop #sep #value #end
                        }
                    }
                    ClassType::Object(indice) => {
                        let cond = &condition_idents[*indice];
                        let value = quote! {
                            if #cond {
                                #value
                            } else {
                                Default::default()
                            }
                        };

                        quote! {
                            #prop #sep #value #end
                        }
                    }
                }
            })
            .collect();

        let token = match &macro_type {
            MacroType::Create => {
                quote! {
                    bevy::ui::Node {
                        #(#props)*
                        ..Default::default()
                    }
                }
            }
            MacroType::Mutate(expr) => {
                quote! {
                    {
                        let __node = &mut #expr;
                        #(__node.#props)*
                    }
                }
            }
        };

        Some(token)
    }
}
