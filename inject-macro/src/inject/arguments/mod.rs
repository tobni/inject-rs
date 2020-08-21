use std::collections::HashSet;

use argument::{DefaultArgs, InjectArgument, Mergable, NoInjectArgs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Expr, Ident, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::inject::input::InjectableSignature;

mod argument;
pub mod error;

pub(crate) struct InjectArgs {
    default_args: Option<DefaultArgs>,
    no_inject_args: Option<NoInjectArgs>,
}

impl InjectArgs {
    pub fn expand_signature(
        mut self,
        sig: &dyn InjectableSignature,
    ) -> Result<Vec<Option<TokenStream>>> {
        let mut args = vec![];
        let mut fields = vec![];
        for argument in sig.inputs() {
            match argument {
                syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                    if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = pat.as_ref() {
                        let default_arg = self.get_default(&ident);
                        let should_inject = !self.is_no_inject(&ident);
                        fields.push(format!("'{}'", ident));
                        let injected_arg =
                            Some(quote! { ::inject::get!(container, #ty, create: #should_inject) })
                                .map(|injection| {
                                    if default_arg.is_some() {
                                        quote! { #injection.or_else(|_| Ok(#default_arg) ) }
                                    } else {
                                        injection
                                    }
                                })
                                .map(|injection| quote! { #injection? });

                        args.push(injected_arg)
                    }
                }
                syn::FnArg::Receiver(receiver) => {
                    return Err(Error::new(
                        receiver.self_token.span(),
                        "not allowed to reference 'self'",
                    ))
                }
            }
        }

        if let Some(&extra_field) = self.remaining().iter().next() {
            return Err(Error::new(
                extra_field.span(),
                format!("unknown identifier, expected {}", fields.join(", ")),
            ));
        }

        Ok(args)
    }

    fn get_default(&mut self, field: &Ident) -> Option<Expr> {
        self.default_args
            .as_mut()
            .map(|args| args.remove(&field))
            .flatten()
    }

    fn is_no_inject(&mut self, field: &Ident) -> bool {
        self.no_inject_args
            .as_mut()
            .map(|args| args.remove(&field))
            .unwrap_or(false)
    }

    fn remaining(&self) -> HashSet<&Ident> {
        &self.remaining_defaults() | &self.remaining_no_injects()
    }

    fn remaining_defaults(&self) -> HashSet<&Ident> {
        self.default_args
            .as_ref()
            .map(|args| args.fields().collect())
            .unwrap_or_default()
    }

    fn remaining_no_injects(&self) -> HashSet<&Ident> {
        self.no_inject_args
            .as_ref()
            .map(|args| args.fields().collect())
            .unwrap_or_default()
    }
}

impl Parse for InjectArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut default_args = vec![];
        let mut no_inject_args = vec![];

        let parsed_arguments: Punctuated<InjectArgument, Token![,]> =
            input.parse_terminated(InjectArgument::parse)?;

        for arg in parsed_arguments {
            match arg {
                InjectArgument::Default { args, .. } => default_args.push(args),
                InjectArgument::NoInject { args, .. } => no_inject_args.push(args),
            }
        }

        let default_args = Mergable::merge_many(default_args)?;
        let no_inject_args = Mergable::merge_many(no_inject_args)?;

        Ok(InjectArgs {
            default_args,
            no_inject_args,
        })
    }
}
