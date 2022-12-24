use proc_macro2::Ident;
use syn::{DeriveInput, Generics, DataStruct,Result};

use crate::attr::{data_type, self};

use super::field::Field;



#[derive(Clone, Debug)]
pub struct Struct<'a> {
    pub original: &'a DeriveInput,
    pub attrs: data_type::Attrs<'a>,
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
}
