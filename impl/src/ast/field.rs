use proc_macro2::{Ident, TokenStream};
use syn::{Fields, Index, Member, Result, Type, TypePath};

use crate::attr::{
    self,
    attr::Attrs,
    field::{self, params::Params},
    mapping_strategy::MappingStrategy, to::To,
};

#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: Attrs<To<'a, Params>>,
    pub member: Member,
    pub ty: &'a Type,
}

impl<'a> Field<'a> {
    pub fn multiple_from_syn(fields: &'a Fields) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field))
            .collect()
    }

    pub fn from_syn(i: usize, node: &'a syn::Field) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: attr::field::get(node)?,
            member: node
                .ident
                .clone()
                .map(Member::Named)
                .unwrap_or_else(|| Member::Unnamed(Index::from(i))),
            ty: &node.ty,
        })
    }

}
