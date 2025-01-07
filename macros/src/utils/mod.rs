use std::{
    any::Any,
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::{ClassType, MacroType};
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, Ident};

pub mod color;
pub mod val;

pub fn parse_neg(str: &str) -> (bool, &str) {
    let neg = str.starts_with('-');
    let str = if neg { &str[1..] } else { str };
    (neg, str)
}

#[derive(Default)]
pub struct StructualTokenStream(pub Vec<(&'static str, TokenStream)>);

impl Deref for StructualTokenStream {
    type Target = Vec<(&'static str, TokenStream)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StructualTokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait ToTokenStream {
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        None
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream;

    // This only used for nested structs in mutation macro, this seem to be a bad design, but I don't have a better idea
    fn structual_to_token_stream(&self) -> Option<StructualTokenStream> {
        None
    }
}

impl ToTokenStream for TokenStream {
    fn to_token_stream(&self) -> TokenStream {
        self.clone()
    }
}

impl ToTokenStream for f32 {
    fn to_token_stream(&self) -> TokenStream {
        quote! {#self}
    }
}

pub enum StructPropValueType {
    Simple(TokenStream),
    Nested(Box<dyn ToTokenStream + Send + Sync + 'static>),
    Wrapped(Box<dyn ToTokenStream + Send + Sync + 'static>),
}

impl StructPropValueType {
    pub fn downcast_mut<T: 'static>(&mut self) -> &mut T {
        let val = match self {
            StructPropValueType::Nested(value) => value,
            StructPropValueType::Wrapped(value) => value,
            _ => panic!("downcast_mut called on non-dynamic StructPropValueType"),
        };

        val.as_any_mut().unwrap().downcast_mut::<T>().unwrap()
    }
}
pub struct StructPropValue {
    pub class_type: ClassType,
    pub value: StructPropValueType,
}

impl StructPropValue {
    pub fn simple(class_type: ClassType, value: impl ToTokenStream) -> Self {
        Self {
            class_type,
            value: StructPropValueType::Simple(value.to_token_stream()),
        }
    }

    pub fn nested(
        class_type: ClassType,
        value: impl ToTokenStream + Send + Sync + 'static,
    ) -> Self {
        Self {
            class_type,
            value: StructPropValueType::Nested(Box::new(value)),
        }
    }

    pub fn wrapped(
        class_type: ClassType,
        value: impl ToTokenStream + Send + Sync + 'static,
    ) -> Self {
        Self {
            class_type,
            value: StructPropValueType::Wrapped(Box::new(value)),
        }
    }
}

impl ToTokenStream for StructPropValue {
    fn to_token_stream(&self) -> TokenStream {
        match &self.value {
            StructPropValueType::Simple(value) => value.clone(),
            StructPropValueType::Nested(value) => value.to_token_stream(),
            StructPropValueType::Wrapped(value) => value.to_token_stream(),
        }
    }
}

pub(crate) struct StructProps<T: AsRef<str>>(pub IndexMap<T, StructPropValue>);

impl<T: AsRef<str>> Default for StructProps<T> {
    fn default() -> Self {
        Self(IndexMap::new())
    }
}

impl<T: AsRef<str>> Deref for StructProps<T> {
    type Target = IndexMap<T, StructPropValue>;

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
        let props = self.0.iter().map(|(prop, value)| {
            let prop = quote_prop(prop.as_ref());
            let class_type = value.class_type;
            let value = value.to_token_stream();
            match class_type {
                ClassType::String => {
                    quote! {
                        #prop: #value
                    }
                }
                ClassType::Object(indice) => {
                    let cond = &condition_idents[indice];
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
            .partition(|(_, value)| matches!(value.class_type, ClassType::String));

        let normal_props = normal_props.iter().map(|(prop, value)| {
            let outer_prop = quote_prop(prop.as_ref());
            match &value.value {
                StructPropValueType::Simple(value) => {
                    quote! {
                        __comp.#outer_prop = #value;
                    }
                }
                StructPropValueType::Wrapped(value) => {
                    let value = value.to_token_stream();
                    quote! {
                        __comp.#outer_prop = #value;
                    }
                }
                StructPropValueType::Nested(value) => {
                    let value = value
                        .structual_to_token_stream()
                        .expect("Nested value must be implement structual_to_token_stream");

                    let stmts = value.0.into_iter().map(|prop| {
                        let value = prop.1;
                        if prop.0.ends_with("()") {
                            let prop = Ident::new(prop.0.trim_end_matches("()"), Span::call_site());

                            quote! {
                                __comp.#outer_prop.#prop(#value);
                            }
                        } else {
                            let prop = quote_prop(&prop.0);
                            quote! {
                                __comp.#outer_prop.#prop = #value;
                            }
                        }
                    });

                    quote! {
                        #(#stmts)*
                    }
                }
            }
        });

        let conditional_props = {
            let mut group = HashMap::new();
            for (prop, value) in conditional_props {
                let class_type = value.class_type;
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
                let cond = &condition_idents[indice];

                let assign_stmts = props.into_iter().flat_map(|(prop, value)| {
                    let outer_prop = quote_prop(prop.as_ref());

                    match &value.value {
                        StructPropValueType::Simple(value) => {
                            vec![quote! {
                                __comp.#outer_prop = #value;
                            }]
                        }
                        StructPropValueType::Wrapped(value) => {
                            let value = value.to_token_stream();
                            vec![quote! {
                                __comp.#outer_prop = #value;
                            }]
                        }
                        StructPropValueType::Nested(value) => {
                            let value = value.structual_to_token_stream().unwrap();

                            value
                                .0
                                .into_iter()
                                .map(|prop| {
                                    let value = prop.1;
                                    if prop.0.ends_with("()") {
                                        let prop = Ident::new(
                                            prop.0.trim_end_matches("()"),
                                            Span::call_site(),
                                        );
                                        quote! {
                                            __comp.#outer_prop.#prop(#value);
                                        }
                                    } else {
                                        let prop = quote_prop(&prop.0);
                                        quote! {
                                            __comp.#outer_prop.#prop = #value;
                                        }
                                    }
                                })
                                .collect()
                        }
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

fn quote_prop(prop: &str) -> TokenStream {
    match prop.parse::<usize>() {
        Ok(field) => {
            let prop = syn::Index::from(field);
            quote! {#prop}
        }
        Err(_) => {
            let prop = Ident::new(prop, Span::call_site());
            quote! {#prop}
        }
    }
}
