use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeMap, HashSet},
    hash::Hash,
};

use crate::{ClassType, ParseCtx};
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub struct QuoteCtx<'a> {
    pub condition_idents: &'a [TokenStream],
    pub is_create: bool,
    pub parent_props: Vec<String>,
    pub parse_ctx: &'a ParseCtx,
}

pub trait ToTokenStream {
    fn to_token_stream(&self) -> TokenStream;
}

impl ToTokenStream for TokenStream {
    fn to_token_stream(&self) -> TokenStream {
        self.clone()
    }
}

pub trait Quote {
    fn quote(&self, ctx: &mut QuoteCtx) -> TokenStream;
}

impl<T> Quote for T
where
    T: ToTokenStream,
{
    fn quote(&self, _: &mut QuoteCtx) -> TokenStream {
        self.to_token_stream()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct StructValPrioritizedKey {
    priority: u8,
    class_type: ClassType,
}

pub struct StructValPrioritized {
    values: BTreeMap<Reverse<StructValPrioritizedKey>, Box<dyn Quote>>,
    use_setter: bool,
}

impl PartialOrd for StructValPrioritizedKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StructValPrioritizedKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = self.priority.cmp(&other.priority);

        match order {
            Ordering::Equal => match (self.class_type, other.class_type) {
                (ClassType::String, ClassType::String) => Ordering::Equal,
                (ClassType::Object(a), ClassType::Object(b)) => a.cmp(&b),
                (ClassType::Object(_), ClassType::String) => Ordering::Greater,
                (ClassType::String, ClassType::Object(_)) => Ordering::Less,
            },
            _ => order,
        }
    }
}

impl StructValPrioritized {
    pub fn new(
        value: impl Quote + 'static,
        class_type: ClassType,
        priority: u8,
        use_setter: bool,
    ) -> Self {
        let mut map: BTreeMap<Reverse<StructValPrioritizedKey>, Box<dyn Quote>> = BTreeMap::new();
        map.insert(
            Reverse({
                StructValPrioritizedKey {
                    priority,
                    class_type,
                }
            }),
            Box::new(value),
        );
        StructValPrioritized {
            values: map,
            use_setter,
        }
    }

    pub fn insert(&mut self, value: impl Quote + 'static, class_type: ClassType, priority: u8) {
        self.values.insert(
            Reverse(StructValPrioritizedKey {
                class_type,
                priority,
            }),
            Box::new(value),
        );
    }
}

impl Quote for StructValPrioritized {
    fn quote(&self, ctx: &mut QuoteCtx) -> TokenStream {
        let entries = self.values.iter().collect::<Vec<_>>();
        let mut evaluated_conds = HashSet::new();
        fn quote(
            idx: usize,
            entries: &[(&Reverse<StructValPrioritizedKey>, &Box<dyn Quote>)],
            ctx: &mut QuoteCtx,
            fallback: TokenStream,
            evaluated_conds: &mut HashSet<usize>,
            use_setter: bool,
        ) -> TokenStream {
            let entry = entries[idx];
            let value = entry.1.quote(ctx);

            match entry.0 .0.class_type {
                ClassType::String => {
                    if ctx.is_create {
                        quote! {#value}
                    } else {
                        let lhs = quote_props(&ctx.parent_props);
                        if use_setter {
                            quote! {
                                #lhs(#value);
                            }
                        } else {
                            quote! {
                                #lhs = #value;
                            }
                        }
                    }
                }
                ClassType::Object(indice) => {
                    let cond = &ctx.condition_idents[indice];
                    let evaluated = evaluated_conds.contains(&indice);
                    if !evaluated {
                        evaluated_conds.insert(indice);
                    }
                    if ctx.is_create {
                        let fallback = if idx + 1 < entries.len() {
                            quote(idx + 1, entries, ctx, fallback, evaluated_conds, use_setter)
                        } else {
                            quote! {Default::default()}
                        };

                        if evaluated {
                            return fallback;
                        }

                        return quote! {
                            if #cond {
                                #value
                            } else {
                                #fallback
                            }
                        };
                    }

                    let else_block = if idx + 1 < entries.len() {
                        let fallback =
                            quote(idx + 1, entries, ctx, fallback, evaluated_conds, use_setter);
                        if evaluated {
                            fallback
                        } else {
                            quote! {
                                else {
                                    #fallback
                                }
                            }
                        }
                    } else {
                        TokenStream::new()
                    };

                    if evaluated {
                        return else_block;
                    }

                    let lhs = quote_props(&ctx.parent_props);

                    if use_setter {
                        quote! {
                            if #cond {
                                #lhs(#value);
                            }
                            #else_block
                        }
                    } else {
                        quote! {
                            if #cond {
                                #lhs = #value;
                            }
                            #else_block
                        }
                    }
                }
            }
        }

        return quote(
            0,
            &entries,
            ctx,
            TokenStream::new(),
            &mut evaluated_conds,
            self.use_setter,
        );
    }
}

pub enum StructVal {
    Prioritized(StructValPrioritized),
    Nested(Struct<&'static str>),
    Raw(TokenStream),
}

impl Quote for StructVal {
    fn quote(&self, ctx: &mut QuoteCtx) -> TokenStream {
        match self {
            StructVal::Prioritized(val) => val.quote(ctx),
            StructVal::Nested(val) => val.quote(ctx),
            StructVal::Raw(val) => val.clone(),
        }
    }
}

impl StructVal {
    pub fn prioritized(
        value: impl Quote + 'static,
        class_type: ClassType,
        priority: u8,
        use_setter: bool,
    ) -> Self {
        StructVal::Prioritized(StructValPrioritized::new(
            value, class_type, priority, use_setter,
        ))
    }

    pub fn nested(value: Struct<&'static str>) -> Self {
        StructVal::Nested(value)
    }

    pub fn raw(value: TokenStream) -> Self {
        StructVal::Raw(value)
    }

    pub fn as_priotized_mut(&mut self) -> &mut StructValPrioritized {
        match self {
            StructVal::Prioritized(v) => v,
            _ => panic!("Expected prioritized"),
        }
    }

    pub fn as_nested_mut(&mut self) -> &mut Struct<&'static str> {
        match self {
            StructVal::Nested(v) => v,
            _ => panic!("Expected nested"),
        }
    }

    pub fn as_nested(&self) -> &Struct<&'static str> {
        match self {
            StructVal::Nested(v) => v,
            _ => panic!("Expected nested"),
        }
    }
}

pub struct Struct<K: AsRef<str>> {
    path: TokenStream,
    pub props: IndexMap<K, StructVal>,
    use_setter: bool,
}

impl<K: AsRef<str> + Eq + Hash> Struct<K> {
    pub fn new(path: TokenStream) -> Self {
        Struct {
            path,
            props: IndexMap::new(),
            use_setter: false,
        }
    }

    pub fn use_setter(mut self, use_setter: bool) -> Self {
        self.use_setter = use_setter;
        self
    }

    pub fn insert(
        &mut self,
        prop: K,
        value: impl Quote + 'static,
        class_type: ClassType,
        priority: u8,
    ) {
        if let Some(prop) = self.props.get_mut(&prop) {
            prop.as_priotized_mut().insert(value, class_type, 0);
            return;
        } else {
            self.props.insert(
                prop,
                StructVal::prioritized(value, class_type, priority, self.use_setter),
            );
        }
    }

    pub fn path(&self) -> &TokenStream {
        &self.path
    }
}

impl<K: AsRef<str> + 'static> Quote for Struct<K> {
    fn quote(&self, ctx: &mut QuoteCtx) -> TokenStream {
        if self.props.is_empty() {
            return TokenStream::new();
        }

        if ctx.is_create {
            let props = self.props.iter().map(|(prop, val)| {
                let prop = quote_prop(prop.as_ref());

                let val = val.quote(ctx);

                if self.use_setter {
                    quote! {
                        #prop(#val)
                    }
                } else {
                    quote! {
                        #prop: #val
                    }
                }
            });

            let path = &self.path;

            if self.use_setter {
                return quote! {
                    #path::default()
                    #(.#props)*
                };
            }

            let default = quote! {..Default::default()};

            return quote! {
                #path {
                    #(#props,)*
                    #default
                }
            };
        }

        let mut stmts = vec![];

        for (prop, val) in &self.props {
            ctx.parent_props.push(prop.as_ref().to_string());
            stmts.push(val.quote(ctx));
            ctx.parent_props.pop();
        }

        quote! {
            #(#stmts)*
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

fn quote_props(props: &[String]) -> TokenStream {
    let props = props.iter().map(|prop| quote_prop(prop.as_str()));
    quote! {#(#props).*}
}
