use proc_macro2::Ident;
use syn::{DataStruct, DeriveInput, Generics, Result, TypePath};

use crate::attr::{
    self, attrs::Attrs, data_type::params::Params, mapping_strategy::MappingStrategy, to::To,
};

use super::field::Field;

#[derive(Clone, Debug)]
pub struct Struct<'a> {
    pub original: &'a DeriveInput,
    pub attrs: Attrs<To<'a, Params>>,
    pub ident: Ident,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a>>,
}

impl<'a> Struct<'a> {
    pub fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
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

    pub fn has_strategy_for_destination(
        &self,
        destination: &TypePath,
        strategy: &MappingStrategy,
    ) -> bool {
        self.attrs
            .to
            .has_destination_for_strategy(destination, strategy)
            || self.fields.iter().any(|field| {
                field
                    .attrs
                    .to
                    .has_destination_for_strategy(destination, strategy)
            })
    }
}
