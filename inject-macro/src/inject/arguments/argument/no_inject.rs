use std::collections::HashSet;

use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{Error, Ident, Result, Token};

use crate::inject::arguments::argument::mergable::Mergable;
use crate::inject::arguments::error::duplicate_field_error;

pub struct NoInjectArgs(HashSet<Ident>);

impl NoInjectArgs {
    pub fn fields(&self) -> impl Iterator<Item = &'_ Ident> {
        self.0.iter()
    }

    pub fn remove(&mut self, field: &Ident) -> bool {
        self.0.remove(field)
    }
}

impl Mergable for NoInjectArgs {
    fn merge(mut self, other: Self) -> Result<Self> {
        if let Some(same) = self.0.intersection(&other.0).next() {
            return Err(duplicate_field_error(same));
        }
        self.0.extend(other.0.into_iter());
        Ok(self)
    }
}

impl Parse for NoInjectArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<_, Token![,]> = input.parse_terminated(syn::Ident::parse)?;
        let mut arg_set = HashSet::default();
        for field in fields {
            if arg_set.contains(&field) {
                return Err(Error::new(
                    field.span(),
                    format!("duplicate identifier '{}'", field),
                ));
            } else {
                arg_set.insert(field);
            }
        }
        Ok(NoInjectArgs(arg_set))
    }
}
