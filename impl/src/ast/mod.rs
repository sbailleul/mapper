
use std::{collections::HashSet, fmt::Debug};
use syn::{Data, DeriveInput, Error, Path, Result};

use crate::attr::{
    field::To, mapping_strategy::MappingStrategy,
    spanned_item::SpannedItem,
};

use self::{data_type::Struct, mapping_tree::{MappingTree, MappingType}, mapping_field::MappingField};

pub mod data_type;
pub mod field;
pub mod mapping_tree;
pub mod mapping_field;

#[derive(Debug)]
pub enum Input<'a> {
    Struct(Struct<'a>),
}


impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            _ => Err(Error::new_spanned(node, "Only c style structs are supported")),
        }
    }
}

impl From<Struct<'_>> for HashSet<MappingTree> {
    fn from(value: Struct<'_>) -> Self {
        let mut mapping_trees = HashSet::new();
        initialize_automatic_mapping_trees(&mut mapping_trees, &value);
        for field in &value.fields {
            add_all_fields_to_automatic_mapping_trees(&mut mapping_trees, field);
            for field_to in &field.attrs.to {
                if field_to.params.exclude.1 {
                    remove_excluded_fields_for_mapping_trees(&mut mapping_trees, field_to, field);
                    continue;
                }
                for field_strategy in &field_to.params.strategies {
                    add_mapping_field_for_additive_mapping_trees(
                        &mut mapping_trees,
                        &value,
                        field_to,
                        &field_strategy.1,
                        field,
                    );
                }
                for with in &field_to.params.with {
                    add_with_function(&mut mapping_trees, &value, field_to, with, field);
                }
            }
        }
        mapping_trees
    }
}

fn add_with_function(
    mapping_trees: &mut HashSet<MappingTree>,
    value: &Struct,
    field_to: &To,
    with: &SpannedItem<Path, MappingStrategy>,
    field: &field::Field,
) {
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

fn add_mapping_field_for_additive_mapping_trees(
    mapping_trees: &mut HashSet<MappingTree>,
    value: &Struct,
    field_to: &To,
    field_strategy: &MappingStrategy,
    field: &field::Field,
) {
    let mut mapping_tree = mapping_trees
        .get_or_insert(MappingTree::new(
            value.ident.clone(),
            field_to.params.destination.clone(),
            field_strategy.clone(),
            Some(MappingType::Additive),
        ))
        .clone();
    mapping_tree.mapping_fields.replace(MappingField {
        ty: field.ty.clone(),
        member: field.member.clone(),
        field: field_to.params.field.clone(),
        strategy: field_strategy.clone(),
        with: Option::flatten(
            field_to
                .params
                .with
                .iter()
                .find(|w| &w.1 == field_strategy)
                .map(|w| w.0.clone()),
        ),
    });
    mapping_trees.replace(mapping_tree);
}

fn remove_excluded_fields_for_mapping_trees(
    mapping_trees: &mut HashSet<MappingTree>,
    field_to: &crate::attr::field::To,
    field: &field::Field,
) {
    let mapping_trees_with_excluded_field_removed = mapping_trees
        .iter()
        .filter(|&mapping_tree| mapping_tree.destination == field_to.params.destination)
        .map(|mapping_tree| {
            let mut mapping_tree = mapping_tree.clone();
            mapping_tree.remove_mapping_fields_by_member(&field.member);
            mapping_tree
        })
        .collect::<Vec<MappingTree>>();
    for mapping_tree in mapping_trees_with_excluded_field_removed {
        mapping_trees.replace(mapping_tree);
    }
}

fn add_all_fields_to_automatic_mapping_trees(
    mapping_trees: &mut HashSet<MappingTree>,
    field: &field::Field,
) {
    let automatic_trees = mapping_trees
        .iter()
        .filter(|&mapping_tree| mapping_tree.has_mapping_type(&MappingType::Automatic))
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
}

fn initialize_automatic_mapping_trees(mapping_trees: &mut HashSet<MappingTree>, value: &Struct) {
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
