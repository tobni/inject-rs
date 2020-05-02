use proc_macro2::TokenStream;
use quote::quote;
use std::convert::identity;
use syn::parse_macro_input;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Field;

mod bool_to_option;
mod call;
mod container;
mod get;
mod inject;

use bool_to_option::BoolToOption;
use call::Call;
use container::Container;
use get::Get;
use inject::Inject;

pub(crate) static NO_INJECT: &str = "no_inject";
pub(crate) static DEFAULT: &str = "default";

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

#[proc_macro_derive(DefaultInject, attributes(default, provide, no_inject))]
pub fn default_inject(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = derive_inject(input, true);
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Inject, attributes(default, provide, no_inject))]
pub fn no_default_inject(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = derive_inject(input, false);
    proc_macro::TokenStream::from(expanded)
}

fn derive_inject(input: DeriveInput, implements_default: bool) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data: &Data = &input.data;

    let inject_fields = inject_fields(data, implements_default);

    quote! {
        impl #impl_generics ::inject::Inject for #name #ty_generics #where_clause {}
    }
}

fn inject_fields(data: &Data, implements_default: bool) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => resolve_named_fields(fields, implements_default),
            syn::Fields::Unnamed(fields) => resolve_unnamed_fields(fields),
            syn::Fields::Unit => quote! {},
        },
        _ => unimplemented!(),
    }
}

fn resolve_name(field: &Field) -> TokenStream {
    field
        .ident
        .as_ref()
        .map(|name| quote! { #name: })
        .unwrap_or_else(|| quote! {})
}

fn resolve_named_fields(fields: &syn::FieldsNamed, implements_default: bool) -> TokenStream {
    let mut injected_fields: Vec<Option<TokenStream>> = fields
        .named
        .iter()
        .map(|field: &Field| {
            let name = resolve_name(field);
            let type_ = &field.ty;
            if is_tagged_default(&field.attrs) {
                implements_default.or(quote! { #name Default::default() })
            } else {
                Some(quote! { #name ::inject::get!(container, #type_)? })
            }
        })
        .collect();
    let defaults = injected_fields
        .iter()
        .any(&Option::is_none)
        .and(quote! { ..Default::default() });
    let injected_fields = injected_fields.drain(..).filter_map(identity);

    quote! { { #(#injected_fields ,)* #defaults } }
}

fn resolve_unnamed_fields(fields: &syn::FieldsUnnamed) -> TokenStream {
    let injected_fields = fields.unnamed.iter().map(|field: &Field| {
        let name = resolve_name(field);
        let type_ = &field.ty;
        is_tagged_default(&field.attrs)
            .and(quote! { #name Default::default() })
            .or_else(|| quote! { #name ::inject::get!(container, #type_)? }.into())
    });
    quote! { ( #(#injected_fields ,)* ) }
}

fn is_tagged(attributes: &[Attribute], tag: &'static str) -> bool {
    attributes
        .iter()
        .any(|attribute: &Attribute| attribute.path.is_ident(&quote::format_ident!("{}", tag)))
}

fn is_tagged_default(attributes: &[Attribute]) -> bool {
    is_tagged(attributes, DEFAULT)
}
