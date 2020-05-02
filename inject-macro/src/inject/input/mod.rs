use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result};

mod constructor;
mod free_function;
mod injectable_signature;

use constructor::ConstructorImpl;
use free_function::FreeFunctionImpl;
pub use injectable_signature::InjectableSignature;

pub enum InjectInput {
    Constructor(ConstructorImpl),
    FreeFunction(FreeFunctionImpl),
}

impl InjectInput {
    pub fn name(&self) -> &Ident {
        self.signature().ident()
    }

    pub fn inputs(&self) -> Vec<&Ident> {
        self.signature().input_idents()
    }

    pub fn signature(&self) -> &dyn InjectableSignature {
        match self {
            InjectInput::Constructor(method) => &method.sig,
            InjectInput::FreeFunction(method) => &method.sig,
        }
    }
}

impl Parse for InjectInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.fork().parse::<ConstructorImpl>().is_ok() {
            input
                .parse::<ConstructorImpl>()
                .map(InjectInput::Constructor)
        } else {
            input.parse().map(InjectInput::FreeFunction)
        }
    }
}
