use bool_to_option::BoolToOption;
use call::Call;
use container::Container;
use get::Get;
use inject::Inject;
use syn::parse_macro_input;

mod bool_to_option;
mod call;
mod container;
mod get;
mod inject;

#[proc_macro_attribute]
pub fn inject(
    arguments: proc_macro::TokenStream,
    method: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let inject = match Inject::try_parse(arguments, method) {
        Ok(inject) => inject,
        Err(err) => return err.to_compile_error().into(),
    };
    inject.expand().into()
}

#[proc_macro]
pub fn get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Get).expand().into()
}

#[proc_macro]
pub fn call(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Call).expand().into()
}

#[proc_macro]
pub fn container(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Container).expand().into()
}
