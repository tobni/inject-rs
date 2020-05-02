use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parenthesized, Result};

mod default;
mod mergable;
mod no_inject;

pub use default::DefaultArgs;
pub use mergable::Mergable;
pub use no_inject::NoInjectArgs;

mod kw {
    syn::custom_keyword!(default);
    syn::custom_keyword!(no_inject);
}

#[allow(dead_code)]
pub enum InjectArgument {
    Default {
        keyword: kw::default,
        paren: Paren,
        args: DefaultArgs,
    },
    NoInject {
        keyword: kw::no_inject,
        paren: Paren,
        args: NoInjectArgs,
    },
}

impl Parse for InjectArgument {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(kw::default) {
            Self::Default {
                keyword: input.parse()?,
                paren: parenthesized!(content in input),
                args: content.parse()?,
            }
        } else if lookahead.peek(kw::no_inject) {
            Self::NoInject {
                keyword: input.parse()?,
                paren: parenthesized!(content in input),
                args: content.parse()?,
            }
        } else {
            return Err(lookahead.error());
        })
    }
}
