use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{Error, Expr, Ident, Result, Token};

use crate::inject::arguments::argument::mergable::Mergable;
use crate::inject::arguments::error::duplicate_field_error;

pub struct DefaultArgs(HashMap<Ident, Expr>);

impl DefaultArgs {
    pub fn remove(&mut self, field: &Ident) -> Option<Expr> {
        self.0.remove(field)
    }

    pub fn field_set(&self) -> HashSet<Ident> {
        self.fields().cloned().collect()
    }
    pub fn fields(&self) -> impl Iterator<Item = &'_ Ident> {
        self.0.keys()
    }
}

impl Parse for DefaultArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut arg_map = HashMap::default();
        let args: Punctuated<DefaultArg, Token![,]> = input.parse_terminated(DefaultArg::parse)?;
        for arg in args {
            match arg_map.entry(arg.field.clone()) {
                Entry::Vacant(entry) => entry.insert(
                    arg.value
                        .unwrap_or_else(|| syn::parse_str("Default::default()").unwrap()),
                ),
                _ => {
                    return Err(Error::new(
                        arg.field.span(),
                        format!("duplicate identifier '{}'", arg.field),
                    ))
                }
            };
        }
        Ok(DefaultArgs(arg_map))
    }
}

impl Mergable for DefaultArgs {
    fn merge(mut self, other: Self) -> Result<Self> {
        if let Some(same) = self.field_set().intersection(&other.field_set()).next() {
            return Err(duplicate_field_error(same));
        }
        self.0.extend(other.0.into_iter());
        Ok(self)
    }
}

struct DefaultArg {
    pub field: Ident,
    pub eq: Option<Token![=]>,
    pub value: Option<Expr>,
}

impl Parse for DefaultArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field = input.parse()?;
        let eq: Option<Token![=]> = input.parse()?;
        let value = if eq.is_some() {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(DefaultArg { field, eq, value })
    }
}
