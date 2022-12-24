use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{ast::MappingField, attr::mapping_strategy::MappingStrategy};


impl MappingField {
    pub fn get_dest_field(&self) -> TokenStream  {
        if let Some(field) = &self.field {
            field.into_token_stream()
        } else {
            (&self.member).into_token_stream()
        }
    }
    pub fn get_src_field(&self) -> TokenStream  {
        let src = &self.member;
        if let Some(with) = &self.with{
            match self.strategy{
                MappingStrategy::Into => quote::quote!(#with(self.#src)),
                MappingStrategy::Mapper => quote::quote!{#with(&self.#src)},
            }
        }else{
            match self.strategy{
                MappingStrategy::Into => quote::quote!(self.#src),
                MappingStrategy::Mapper => quote::quote!{self.#src.clone()},
            }
        }
    }
}
