use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parenthesized, Attribute, Block, FnArg, Generics, Ident, Result, Token, Visibility};

use crate::inject::input::InjectableSignature;

pub struct ConstructorImpl {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Option<Token![default]>,
    pub sig: Constructor,
    pub block: Block,
}

impl Parse for ConstructorImpl {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: Attribute::parse_outer(input)?,
            vis: input.parse()?,
            defaultness: input.parse()?,
            sig: input.parse()?,
            block: input.parse()?,
        })
    }
}

impl InjectableSignature for Constructor {
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

pub struct Constructor {
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub generics: Generics,
    pub paren: Paren,
    pub inputs: Punctuated<FnArg, Token![,]>,
    pub arrow: Token![->],
    pub output: Token![Self],
}

impl Parse for Constructor {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            fn_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            paren: parenthesized!(content in input),
            inputs: Punctuated::parse_terminated(&content)?,
            arrow: input.parse()?,
            output: input
                .parse()
                .or_else(|_| Err(input.error("expected 'Self'")))?,
        })
    }
}
