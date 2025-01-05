use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::{ClassType, MacroType};
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, Ident, Index};

pub mod val;

pub fn parse_neg(str: &str) -> (bool, &str) {
    let neg = str.starts_with('-');
    let str = if neg { &str[1..] } else { str };
    (neg, str)
}

pub trait IntoTokenStream {
    fn into_token_stream(self) -> proc_macro2::TokenStream;
}

pub(crate) struct StructProps<T: AsRef<str>>(
    pub IndexMap<T, (proc_macro2::TokenStream, ClassType)>,
);

impl<T: AsRef<str>> Default for StructProps<T> {
    fn default() -> Self {
        Self(IndexMap::new())
    }
}

impl<T: AsRef<str>> Deref for StructProps<T> {
    type Target = IndexMap<T, (proc_macro2::TokenStream, ClassType)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: AsRef<str>> DerefMut for StructProps<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: AsRef<str> + Hash + Eq> StructProps<T> {
    pub fn quote(
        &self,
        path: TokenStream,
        condition_idents: &[TokenStream],
        macro_type: &MacroType,
    ) -> Option<TokenStream> {
        if self.0.is_empty() {
            return None;
        }

        let token_stream = match macro_type {
            MacroType::Create => self.quote_creation_code(path, condition_idents),
            MacroType::Mutate(expr) => self.quote_mutation_code(condition_idents, expr),
        };

        Some(token_stream)
    }

    fn quote_creation_code(
        &self,
        path: TokenStream,
        condition_idents: &[TokenStream],
    ) -> TokenStream {
        let props = self.0.iter().map(|(prop, (value, class_type))| {
            let prop = quote_field(prop.as_ref());
            match class_type {
                ClassType::String => {
                    quote! {
                        #prop: #value
                    }
                }
                ClassType::Object(indice) => {
                    let cond = &condition_idents[*indice];
                    quote! {
                        #prop: if #cond {
                            #value
                        } else {
                            Default::default()
                        }
                    }
                }
            }
        });

        quote! {
            #path {
                #(#props,)*
                ..Default::default()
            }
        }
    }

    fn quote_mutation_code(&self, condition_idents: &[TokenStream], expr: &Expr) -> TokenStream {
        let (normal_props, conditional_props): (Vec<_>, Vec<_>) = self
            .0
            .iter()
            .partition(|(_, (_, class_type))| matches!(class_type, ClassType::String));

        let normal_props = normal_props.iter().map(|(prop, (value, _))| {
            let prop = quote_field(prop.as_ref());

            quote! {
                __comp.#prop = #value;
            }
        });

        let conditional_props = {
            let mut group = HashMap::new();
            for (prop, (value, class_type)) in conditional_props {
                let indice = match class_type {
                    ClassType::Object(indice) => indice,
                    _ => unreachable!(),
                };
                group
                    .entry(indice)
                    .or_insert_with(Vec::new)
                    .push((prop, value));
            }

            group.into_iter().map(|(indice, props)| {
                let cond = &condition_idents[*indice];

                let assign_stmts = props.into_iter().map(|(prop, value)| {
                    let prop = quote_field(prop.as_ref());

                    quote! {
                        __comp.#prop = #value;
                    }
                });

                quote! {
                    if #cond {
                        #(#assign_stmts)*
                    }
                }
            })
        };

        quote! {
            let __comp = &mut #expr;
            #(#normal_props)*
            #(#conditional_props)*
        }
    }
}

fn quote_field(field: &str) -> TokenStream {
    match field.parse::<usize>() {
        Ok(field) => {
            let field = syn::Index::from(field);
            quote! {#field}
        }
        Err(_) => {
            let field = Ident::new(field, Span::call_site());
            quote! {#field}
        }
    }
}
