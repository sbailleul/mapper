use proc_macro2::{Ident, TokenStream};
use std::{collections::HashSet, fmt::Debug, hash::Hash, ops::Deref, rc::Rc};
use syn::{Data, DeriveInput, Error, Member, Path, Result, Type, TypePath};

use crate::attr::{mapping_strategy::MappingStrategy, data_type::AggregatedTo};

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
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum MappingType {
    Additive,
    Automatic,
}
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct MappingField {
    pub ty: Type,
    pub strategy: MappingStrategy,
    pub member: Member,
    pub field: Option<Path>,
    pub with: Option<Path>,
}

#[derive(Eq, Debug, Clone)]
pub struct MappingTree {
    pub ident: Ident,
    pub destination: TypePath,
    pub strategy: MappingStrategy,
    pub mapping_fields: HashSet<MappingField>,
    pub mapping_type: Option<MappingType>,
}

impl PartialEq for MappingTree {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
            && self.destination == other.destination
            && self.strategy == other.strategy
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
    pub fn new(
        ident: Ident,
        destination: TypePath,
        strategy: MappingStrategy,
        mapping_type: Option<MappingType>,
    ) -> Self {
        Self {
            destination,
            ident,
            mapping_fields: HashSet::new(),
            strategy,
            mapping_type,
        }
    }
}
impl MappingTree {
    pub fn remove_mapping_fields_by_member(&mut self, member: &Member) {
        let fields = self
            .mapping_fields
            .iter()
            .filter(|&field| &field.member == member)
            .map(|mapping_field| mapping_field.clone())
            .collect::<Vec<MappingField>>();
        for field in fields {
            self.mapping_fields.remove(&field);
        }
    }
    fn has_mapping_type(&self, mapping_type: &MappingType) -> bool {
        self
            .mapping_type
            .is_some_and(|mapping_tree_type| mapping_tree_type == mapping_type)
    }
}

impl From<Struct<'_>> for HashSet<MappingTree> {
    fn from(value: Struct<'_>) -> Self {
        let mut mapping_trees = HashSet::new();
        initialize_automatic_mapping_trees(&mut mapping_trees, &value);
        for field in &value.fields {
            let automatic_trees = mapping_trees
                .iter()
                .filter(|&mapping_tree| {
                    mapping_tree.has_mapping_type(&MappingType::Automatic) && field.attrs.to.is_empty()
                })
                .map(|mapping_tree| {
                    let mut mapping_tree = mapping_tree.clone();
                    mapping_tree.mapping_fields.replace(MappingField {
                        ty: field.ty.clone(),
                        strategy: mapping_tree.strategy.clone(),
                        member: field.member.clone(),
                        field: None,
                        with: None,
                    });
                    mapping_tree
                })
                .collect::<Vec<MappingTree>>();
            for automatic_tree in automatic_trees {
                mapping_trees.replace(automatic_tree);
            }
            for field_to in &field.attrs.to {
                if field_to.params.exclude.1 {
                    let mapping_trees_with_exluded_field_removed = mapping_trees
                        .iter()
                        .filter(|&mapping_tree| {
                            mapping_tree.destination == field_to.params.destination
                        })
                        .map(|mapping_tree| {
                            let mut mapping_tree = mapping_tree.clone();
                            mapping_tree.remove_mapping_fields_by_member(&field.member);
                            mapping_tree
                        })
                        .collect::<Vec<MappingTree>>();
                    for mapping_tree in mapping_trees_with_exluded_field_removed {
                        mapping_trees.replace(mapping_tree);
                    }
                    continue;
                }
                for field_strategy in &field_to.params.strategies {
                    let mut mapping_tree = mapping_trees
                        .get_or_insert(MappingTree::new(
                            value.ident.clone(),
                            field_to.params.destination.clone(),
                            field_strategy.1.clone(),
                            Some(MappingType::Additive),
                        ))
                        .clone();
                    mapping_tree.mapping_fields.replace(MappingField {
                        ty: field.ty.clone(),
                        member: field.member.clone(),
                        field: field_to.params.field.clone(),
                        strategy: field_strategy.1.clone(),
                        with: Option::flatten(
                            field_to
                                .params
                                .with
                                .iter()
                                .find(|&w| w.1 == field_strategy.1)
                                .map(|w| w.0.clone()),
                        ),
                    });
                    mapping_trees.replace(mapping_tree);
                }
                for with in &field_to.params.with {
                    let mut mapping_tree = mapping_trees
                        .get(&MappingTree::new(
                            value.ident.clone(),
                            field_to.params.destination.clone(),
                            with.1.clone(),
                            None,
                        ))
                        .unwrap()
                        .clone();

                    mapping_tree.mapping_fields.replace(MappingField {
                        ty: field.ty.clone(),
                        member: field.member.clone(),
                        field: field_to.params.field.clone(),
                        strategy: with.1.clone(),
                        with: Option::flatten(
                            field_to
                                .params
                                .with
                                .iter()
                                .find(|&w| w.1 == with.1)
                                .map(|w| w.0.clone()),
                        ),
                    });
                    mapping_trees.replace(mapping_tree);
                }
            }
        }
        mapping_trees
    }
}



fn initialize_automatic_mapping_trees( mapping_trees: &mut HashSet<MappingTree>, value: &Struct) {
    let struct_to = &value.attrs.to;
    for (strategy, destinations) in &struct_to.destinations_by_strategy {
        for destination in destinations {
            mapping_trees.insert(MappingTree::new(
                value.ident.clone(),
                destination.clone(),
                strategy.1.clone(),
                Some(MappingType::Automatic),
            ));
        }
    }
}
