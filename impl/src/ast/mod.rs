use proc_macro2::Ident;
use std::{collections::HashSet, fmt::Debug, hash::Hash, rc::Rc};
use syn::{Data, DeriveInput, Error, Member, Path, Result, TypePath};

use crate::attr::mapping_strategy::MappingStrategy;

use self::data_type::Struct;

pub mod data_type;
pub mod field;

#[derive(Debug)]
pub enum Input<'a> {
    Struct(Struct<'a>),
}

impl Debug for Struct<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct").finish()
    }
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            _ => Err(Error::new_spanned(node, "Only structs are supported")),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct MappingField {
    pub ty: TypePath,
    pub member: Member,
    pub field: Option<Path>,
    pub with: Option<Path>,
}

#[derive(Eq, Debug)]
pub struct MappingTree {
    pub ident: Rc<Ident>,
    pub destination: Rc<TypePath>,
    pub strategy: Rc<MappingStrategy>,
    pub mapping_fields: HashSet<MappingField>,
}

impl PartialEq for MappingTree {
    fn eq(&self, other: &Self) -> bool {
        self.destination == other.destination && self.strategy == other.strategy
    }
}

impl Hash for MappingTree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.destination.hash(state);
        self.strategy.hash(state);
    }
}

impl MappingTree {
    pub fn new(ident: Rc<Ident>, destination: Rc<TypePath>, strategy: Rc<MappingStrategy>) -> Self {
        Self {
            destination,
            ident,
            mapping_fields: HashSet::new(),
            strategy,
        }
    }
}

// impl TryFrom<Struct<'_>> for HashSet<MappingTree> {
//     type Error = syn::Error;

//     fn try_from(value: Struct<'_>) -> Result<Self> {
//         let mut mapping_trees: Self = Self::new();
//         let struct_ident = Rc::new(value.ident);
//         for to_attr in value.attrs.to.to {
//             for dest in to_attr.params.destinations {
//                 for strategy in to_attr.params.strategies {
//                     let fields = value.fields
//                     .iter()
//                     .flat_map(|field| field.attrs.to
//                         .iter()
//                         .filter(|&to| to.params.destination == *dest && !to.params.exclude)
//                         .map(|to| MappingField{ty:field.ty, member: field.member, field: to.params.field, with: to.params.with})
//                     );
//                     mapping_trees.insert(MappingTree::new(struct_ident, dest, strategy));
//                 }
//             }
//         }
//         for field in value.fields{
//             for to_attr in field.attrs.to {
//                 if to_attr.params.with.is_empty(){
//                     1
//                 }
//                 for ele in to_attr.params.with {
                    
//                 }
//                 mapping_trees.insert(value)to_attr.params.destination
//             }
//         } 

//         // for field in value.fields {
//         // for field_to in field.attrs.to {
//         // field_to.params.destination;
//         // mapping_trees.iter().any(|mt| mt.destination == mt.destination && mt.)
//         // }
//         // }
//         let mapping_trees = vec![];
//         mapp
//     }
// }
