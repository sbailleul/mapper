mod ast;
mod attr;
mod expand;
mod generics;
mod valid;

extern crate proc_macro;

use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Mapper, attributes(to))]
pub fn derive_into(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // let attributes = input.attrs.iter().filter(|a|);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
