use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitBool, Path, Result, Token};

use crate::bool_to_option::BoolToOption;

mod kw {
    syn::custom_keyword!(create);
}

pub struct Create {
    pub keyword: kw::create,
    pub colon: Token![:],
    pub boolean: LitBool,
}

impl Parse for Create {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            colon: input.parse()?,
            boolean: input.parse()?,
        })
    }
}

pub struct Get {
    pub expr: Expr,
    pub comma: Token![,],
    pub ampersand: Option<Token![&]>,
    pub ident: Path,
    pub comma2: Option<Token![,]>,
    pub create: Option<Create>,
}

impl Get {
    pub fn expand(self) -> TokenStream {
        let Get {
            ident,
            expr,
            ampersand,
            create,
            ..
        } = self;
        let can_create = if let Some(create) = create {
            create.boolean.value
        } else {
            true
        };

        let can_fallback = if let Some(segment) = ident.segments.last() {
            (segment.ident != "Arc") && can_create
        } else {
            false
        };

        let fallback = can_fallback.and_then(|| {
            quote! {.or_else(|_| <#ident>::inject(#expr))}
        });

        if ampersand.is_none() {
            quote! {
                {
                    use ::inject::{Inject, InjectExt};
                    (#expr)
                        .get::<#ident>()
                        #fallback

                }
            }
        } else {
            quote! {
                {
                    use ::inject::{Inject, InjectExt};
                    (#expr)
                        .get_ref::<#ident>()
                }
            }
        }
    }
}

impl Parse for Get {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse()?;
        let comma = input.parse()?;
        let ampersand = input.parse()?;
        let ident: Path = input.parse()?;
        let comma2 = input.parse()?;
        let create = (!input.is_empty()).and_then(|| input.parse()).transpose()?;

        Ok(Self {
            expr,
            comma,
            ampersand,
            ident,
            comma2,
            create,
        })
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use syn::parse2;

    use super::Get;

    #[test]
    fn test_parsing_args() {
        let tree = quote! {
            &container, A<isize>
        };

        let get: Get = parse2(tree).unwrap();

        assert_eq!(get.ident.to_token_stream().to_string(), "A < isize >");
        assert_eq!(get.comma.to_token_stream().to_string(), ",");
        assert_eq!(get.expr.to_token_stream().to_string(), "& container");
    }

    #[test]
    fn test_expansion() {
        let tree = quote! {
            &container, A<isize>
        };

        let expected = quote! {
            {
                use ::inject::{Inject, InjectExt};
                (&container)
                    .get::<A<isize > >()
                    .or_else(|_| < A < isize > >::inject(& container ) )
            }
        };

        let get: TokenStream = parse2::<Get>(tree).unwrap().expand();

        assert_eq!(get.to_string(), expected.to_string())
    }

    #[test]
    fn test_arc_expansion() {
        let tree = quote! {
            &container, Arc<A<isize>>
        };

        let expected = quote! {
            {
                use ::inject::{Inject, InjectExt};
                (&container).get::<Arc<A<isize > > >()
            }
        };

        let get: TokenStream = parse2::<Get>(tree).unwrap().expand();

        assert_eq!(get.to_string(), expected.to_string())
    }
}
