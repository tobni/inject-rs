use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse, Ident, Result};

use arguments::InjectArgs;
use input::InjectInput;

pub mod arguments;
mod input;

pub struct Inject {
    origin: TokenStream,
    args: InjectArgs,
    method: InjectInput,
}

impl Inject {
    pub fn try_parse(
        arguments: proc_macro::TokenStream,
        method: proc_macro::TokenStream,
    ) -> Result<Self> {
        let origin = proc_macro2::TokenStream::from(method.clone());
        let method = parse(method)?;
        let args = parse(arguments)?;

        Ok(Self {
            origin,
            args,
            method,
        })
    }

    pub fn expand(self) -> TokenStream {
        let Self {
            origin,
            args,
            method,
        } = self;

        let name = method.name();
        let inputs = method.inputs();
        let args = match args.expand_signature(method.signature()) {
            Ok(parsed_args) => parsed_args,
            Err(compile_error) => return compile_error.to_compile_error(),
        };

        let expansion = match method {
            InjectInput::Constructor(_) => Self::expand_constructor(name, args),
            InjectInput::FreeFunction(_) => Self::expand_free_function(name, inputs, args),
        };

        quote! {
            #origin
            #expansion
        }
    }

    fn expand_constructor(name: &Ident, args: Vec<Option<TokenStream>>) -> TokenStream {
        quote! {
            pub fn inject(container: &::inject::Container) -> Result<Self, ::inject::InjectError> {
                Ok(
                    Self:: #name ( #(#args,)* )
                )
            }
        }
    }

    fn expand_free_function(
        name: &Ident,
        inputs: Vec<&Ident>,
        args: Vec<Option<TokenStream>>,
    ) -> TokenStream {
        let macro_name = format_ident!("__inject_{}", name);
        let macro_expansion = format_ident!("{}_expand", macro_name);

        let expand_macro = quote! {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! #macro_expansion {
                ( ) => { };
                #( ( $container:expr, #inputs #inputs : $arg:expr ) => {
                    $arg
                };
                ($container:expr, #inputs) => {
                    {
                        let container = $container;
                        #args
                    }

                };
            )*}
        };

        let inject_macro = quote! {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! #macro_name {
                ($container:expr #( $(, #inputs : $#inputs:expr )?  )* ) => {
                    {
                        let __helper = |container: &::inject::Container| {
                            Ok(#name ( #( #macro_expansion ! ( $container, #inputs $( #inputs : $#inputs )? ) ,)* ))
                        };
                        let result: Result<_, ::inject::InjectError> = __helper($container);
                        result
                    }
                };
            }
        };

        quote! {
            #expand_macro
            #inject_macro
        }
    }
}
