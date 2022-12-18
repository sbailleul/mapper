use proc_macro2::{Ident, TokenStream};
use syn::{Member, Type, Fields, Result, Index, TypePath};

use crate::attr::{field, self, mapping_strategy::MappingStrategy};



#[derive(Debug)]
pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: field::Attrs<'a>,
    pub member: Member,
    pub ty: &'a Type
}

impl<'a> Field<'a> {
    pub fn multiple_from_syn(
        fields: &'a Fields
    ) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field))
            .collect()
    }

    pub fn from_syn(
        i: usize,
        node: &'a syn::Field
    ) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: attr::field::get(node)?,
            member: node.ident.clone().map(Member::Named).unwrap_or_else(|| {
                Member::Unnamed(Index::from(i))
            }),
            ty: &node.ty
        })
    }

    pub fn get_destination_field_by_path(&self, path: &TypePath) -> Ident{
        if let Some(field) = self.get_to_by_type(path).and_then(|to| to.params.field.as_ref()){
            field.get_ident().unwrap().clone()
        }else{
            self.original.ident.clone().unwrap()
        }
    }

    pub fn is_excluded(&self, path: &TypePath) -> bool{
        if let Some(field) = self.get_to_by_type(path){
            field.params.exclude
        }else{
            false
        }
    }

    pub fn get_source_value_by_path(&self, path: &TypePath, strategy: &MappingStrategy) -> TokenStream{
        let original = &self.member;
        if let Some(with) = self.get_to_by_type(path).and_then(|to| to.params.with.as_ref()){
            quote::quote!{#with(&self.#original)}
        }else{
            match strategy{
                MappingStrategy::Into => quote::quote!(self.#original),
                MappingStrategy::Mapper => quote::quote!{self.#original.clone()},
            }
        }
    }

    fn get_to_by_type(&self, type_path: &TypePath) -> Option<&field::To> {
        self.attrs.to.iter().find(|&to| to.params.ty.path.get_ident() == type_path.path.get_ident())
    }
}