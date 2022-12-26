pub mod mapping_field;
pub mod mapping_tree;

use std::collections::HashSet;

use proc_macro2::TokenStream;

use syn::DeriveInput;
use syn::Result;

use crate::ast::data_type::Struct;
use crate::ast::mapping_tree::MappingTree;
use crate::ast::Input;

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    input.validate()?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
    })
}

fn impl_struct(input: Struct) -> TokenStream {
    let mapping_trees = HashSet::<MappingTree>::from(input.clone());
    let (_impl_generics, _ty_generics, _where_clause) = input.generics.split_for_impl();
    let mut token_stream = TokenStream::new();
    for mapping_tree in mapping_trees {
        mapping_tree.expand(&mut token_stream)
    }
    token_stream
}
