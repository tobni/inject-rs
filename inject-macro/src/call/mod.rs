use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token};

use crate::BoolToOption;

mod kwargs;

use kwargs::Kwargs;

pub struct Call {
    pub ident: Expr,
    pub comma1: Token![,],
    pub func: Ident,
    pub comma2: Option<Token![,]>,
    pub kwargs: Option<Kwargs>,
}

impl Call {
    pub fn expand(self) -> TokenStream {
        let Call {
            ident,
            func,
            kwargs,
            ..
        } = self;
        let macro_name = format_ident!("__inject_{}", func);
        let fields = kwargs.as_ref().map(|kwargs| {
            let kwargs = kwargs.fields.iter();
            quote! { #(, #kwargs )* }
        });

        quote! { #macro_name ! (#ident #fields) }
    }
}

impl Parse for Call {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            comma1: input.parse()?,
            func: input.parse()?,
            comma2: input.parse()?,
            kwargs: (!input.is_empty()).and_then(|| input.parse()).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Call;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_parsing_args() {
        let tree = quote! {
            &container, a_func
        };

        let call: Call = parse2(tree).unwrap();

        assert!(call.func == "a_func");
    }

    #[test]
    fn test_parsing_args_with_kwargs() {
        let tree = quote! {
            &container, a_func, kwargs = { a: 1 }
        };

        let call: Call = parse2(tree).unwrap();

        assert!(call.func == "a_func");
    }

    #[test]
    fn test_expansion() {
        let tree = quote! {
            &container, a_func
        };

        let call = parse2::<Call>(tree).unwrap().expand();

        let expected = quote! {
            __inject_a_func!(&container)
        };

        assert_eq!(call.to_string(), expected.to_string())
    }

    #[test]
    fn test_expansion_with_kwargs() {
        let tree = quote! {
            &container, a_func, kwargs = { a: 1 }
        };

        let call = parse2::<Call>(tree).unwrap().expand();

        let expected = quote! {
            __inject_a_func!(&container, a: 1)
        };

        assert_eq!(call.to_string(), expected.to_string())
    }
}
