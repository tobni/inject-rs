use syn::punctuated::Punctuated;
use syn::{FnArg, Generics, Ident, Token};

pub trait InjectableSignature {
    fn ident(&self) -> &Ident;

    fn generics(&self) -> &Generics;

    fn inputs(&self) -> &Punctuated<FnArg, Token![,]>;

    fn input_idents(&self) -> Vec<&Ident> {
        self.inputs()
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(syn::PatType { pat, .. }) => Some(pat),
                _ => None,
            })
            .filter_map(|pat| match pat.as_ref() {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => Some(ident),
                _ => None,
            })
            .collect()
    }
}

impl InjectableSignature for syn::Signature {
    fn ident(&self) -> &Ident {
        &self.ident
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn inputs(&self) -> &Punctuated<FnArg, Token![,]> {
        &self.inputs
    }
}
