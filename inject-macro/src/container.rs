use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Result, Token};

pub struct Container {
    providers: Punctuated<Provider, Token![,]>,
}

impl Container {
    pub fn expand(self) -> TokenStream {
        let provider_calls = self.providers.into_iter().map(
            |Provider {
                 ref_token,
                 provider,
             }| match ref_token {
                Some(_) => quote! { container.install_ref(#provider) },
                None => quote! { container.install(#provider) },
            },
        );

        quote! {
            {
                let mut container = ::inject::Container::new();
                #(#provider_calls; )*
                container

            }
        }
    }
}

impl Parse for Container {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            providers: Punctuated::parse_terminated(&input)?,
        })
    }
}

struct Provider {
    ref_token: Option<Token![ref]>,
    provider: Expr,
}

impl Parse for Provider {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ref_token: input.parse()?,
            provider: input.parse()?,
        })
    }
}
