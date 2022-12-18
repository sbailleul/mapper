use std::fmt::Debug;

use proc_macro2::{Span, TokenStream};

use syn::{DeriveInput, Ident, Generics, Member, Type, Result, Data, DataStruct, Error, Fields, Index, spanned::Spanned, Path, TypePath};

use crate::{attr::{ self, data_type, field, mapping_strategy::MappingStrategy}};

#[derive(Debug)]
pub enum Input<'a> {
    Struct(Struct<'a>)
}

pub struct Struct<'a> {
    pub original: &'a DeriveInput,
    pub attrs: data_type::Attrs<'a>,
    pub ident: Ident,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a>>,
}

impl Debug for Struct<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
       .finish()
    }
}

#[derive(Debug)]
pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: field::Attrs<'a>,
    pub member: Member,
    pub ty: &'a Type
}


impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            _ => Err(Error::new_spanned(node, "Only structs are supported")),
        }
    }
}


impl<'a> Struct<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        let attrs = attr::data_type::get(node)?;
        let fields = Field::multiple_from_syn(&data.fields)?;

        Ok(Struct {
            original: node,
            attrs,
            ident: node.ident.clone(),
            generics: &node.generics,
            fields,
        })
    }
}


impl<'a> Field<'a> {
    fn multiple_from_syn(
        fields: &'a Fields
    ) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field))
            .collect()
    }

    fn from_syn(
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

