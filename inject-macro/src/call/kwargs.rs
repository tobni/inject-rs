use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{braced, Expr, Ident, Result, Token};

mod kw {
    syn::custom_keyword!(kwargs);
}

pub struct Kwargs {
    pub keyword: kw::kwargs,
    pub equals: Token![=],
    pub brace: Brace,
    pub fields: Punctuated<Kwarg, Token![,]>,
}

impl Parse for Kwargs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            keyword: input.parse()?,
            equals: input.parse()?,
            brace: braced!(content in input),
            fields: Punctuated::parse_terminated(&content)?,
        })
    }
}

pub struct Kwarg {
    member: Ident,
    colon: Token![:],
    expr: Expr,
}

impl Parse for Kwarg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            member: input.parse()?,
            colon: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for Kwarg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.member.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::{Kwarg, Kwargs};
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_parsing_args() {
        let tree = quote! {
            kwargs = { a: "Hi", b: A::new(), c: P }
        };

        let kwargs: Kwargs = parse2(tree).unwrap();
        let members = vec!["a", "b", "c"];

        assert_eq!(kwargs.fields.len(), members.len());

        for kwarg in kwargs.fields {
            assert!(members.contains(&kwarg.member.to_string().as_str()));
        }
    }

    #[test]
    fn test_to_tokens_for_kwarg() {
        let tree = quote! {
            a: "Hi"
        };

        let kwarg: Kwarg = parse2(tree).unwrap();
        let expected = quote! {
            a: "Hi"
        };

        let expanded = quote! {
            #kwarg
        };

        assert_eq!(expected.to_string(), expanded.to_string())
    }
}
