#![feature(trait_alias)]
#![feature(hash_set_entry)]

mod ast;
mod attr;
mod common;
mod expand;
mod valid;
extern crate proc_macro;

use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Mapper, attributes(to))]
pub fn derive_mapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
