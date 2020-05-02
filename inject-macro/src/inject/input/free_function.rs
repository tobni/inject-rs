use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{
    parenthesized, Attribute, Block, FnArg, Generics, Ident, Result, ReturnType, Token, Type,
    TypePath, Visibility,
};

use crate::inject::input::InjectableSignature;

pub struct FreeFunctionImpl {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: FreeFunction,
    pub block: Block,
}

impl Parse for FreeFunctionImpl {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: Attribute::parse_outer(input)?,
            vis: input.parse()?,
            sig: input.parse()?,
            block: input.parse()?,
        })
    }
}

pub struct FreeFunction {
    pub unsafety: Option<Token![unsafe]>,
    pub asyncness: Option<Token![async]>,
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub generics: Generics,
    pub paren: Paren,
    pub inputs: Punctuated<FnArg, Token![,]>,
    pub output: ReturnType,
}

impl Parse for FreeFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let unsafety = input.parse()?;
        let asyncness = input.parse()?;
        let fn_token = input.parse()?;
        let ident = input.parse()?;
        let generics = input.parse()?;
        let paren = parenthesized!(content in input);
        let inputs = Punctuated::parse_terminated(&content)?;
        let output = input.parse()?;
        if let ReturnType::Type(_, ty) = &output {
            if let Type::Path(TypePath { path, .. }) = ty.as_ref() {
                if let Some(segment) = path.segments.last() {
                    if segment.ident == "Self" {
                        return Err(input.error("'Self' not allowed in free function"));
                    }
                }
            }
        };
        Ok(Self {
            unsafety,
            asyncness,
            fn_token,
            ident,
            generics,
            paren,
            inputs,
            output,
        })
    }
}

impl InjectableSignature for FreeFunction {
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
