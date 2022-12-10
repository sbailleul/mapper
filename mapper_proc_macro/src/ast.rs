use std::fmt::Debug;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{DeriveInput, Ident, Generics, Member, Type, Result, Data, DataStruct, Error, Fields, Index, spanned::Spanned, Path};

use crate::{attr::{ self, data_type, field}, generics::ParamsInScope};

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


pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: field::Attrs,
    pub member: Member,
    pub ty: &'a Type,
    pub contains_generic: bool,
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
        let attrs = attr::data_type::get(&node)?;
        let scope = ParamsInScope::new(&node.generics);
        let span = attrs.span().unwrap_or_else(Span::call_site);
        let fields = Field::multiple_from_syn(&data.fields, &scope, span)?;
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
        fields: &'a Fields,
        scope: &ParamsInScope<'a>,
        span: Span,
    ) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field, scope, span))
            .collect()
    }

    fn from_syn(
        i: usize,
        node: &'a syn::Field,
        scope: &ParamsInScope<'a>,
        span: Span,
    ) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: attr::field::get(&node)?,
            member: node.ident.clone().map(Member::Named).unwrap_or_else(|| {
                Member::Unnamed(Index {
                    index: i as u32,
                    span,
                })
            }),
            ty: &node.ty,
            contains_generic: scope.intersects(&node.ty),
        })
    }

    pub fn get_destination_field_by_path(&self, path: &Path) -> Ident{
        if let Some(field) = self.get_to_by_type(path).and_then(|to| to.field.as_ref()){
            field.get_ident().unwrap().clone()
        }else{
            self.original.ident.clone().unwrap()
        }
    }

    pub fn get_source_value_by_path(&self, path: &Path) -> TokenStream{
        let original = &self.member;
        if let Some(with) = self.get_to_by_type(path).and_then(|to| to.with.as_ref()){
            quote::quote!{#with(self.#original.clone())}
        }else{
            quote::quote!{self.#original.clone()}
        }
    }

    fn get_to_by_type(&self, path: &Path) -> Option<&field::To> {
        self.attrs.to.iter().find(|&to| to.ty.get_ident() == path.get_ident())
    }
}

impl data_type::Attrs<'_> {
    pub fn span(&self) -> Option<Span> {
        Some(self.to.original.span())
    }
}

