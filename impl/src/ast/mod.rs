use std::{collections::HashSet, fmt::Debug};
use syn::{Data, DeriveInput, Error, Path, Result};

use crate::attr::{
    field::params::Params, mapping_strategy::MappingStrategy, spanned_item::SpannedItem, to::To,
};

use self::{
    data_type::Struct,
    mapping_field::MappingField,
    mapping_tree::{MappingTree, MappingType},
};

pub mod data_type;
pub mod field;
pub mod mapping_field;
pub mod mapping_tree;

#[derive(Debug)]
pub enum Input<'a> {
    Struct(Struct<'a>),
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            _ => Err(Error::new_spanned(
                node,
                "Only c style structs are supported",
            )),
        }
    }
}

impl From<Struct<'_>> for HashSet<MappingTree> {
    fn from(value: Struct<'_>) -> Self {
        let mut mapping_trees = HashSet::new();
        initialize_automatic_mapping_trees(&mut mapping_trees, &value);
        for field in &value.fields {
            add_all_fields_to_automatic_mapping_trees(&mut mapping_trees, field);
            for field_to in &field.attrs.to.to_items {
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
                add_non_strategy_dependent_fields_params(&mut mapping_trees, field_to, field);
            }
        }
        mapping_trees
    }
}

fn add_with_function(
    mapping_trees: &mut HashSet<MappingTree>,
    value: &Struct,
    field_to: &To<Params>,
    with: &SpannedItem<Path, MappingStrategy>,
    field: &field::Field,
) {
    if field_to.params.destination.is_none() {
        return;
    }
    let field_dest = field_to.params.destination.as_ref().unwrap();
    let mut mapping_tree = mapping_trees
        .get(&MappingTree::new(
            value.ident.clone(),
            field_dest.clone(),
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

fn add_non_strategy_dependent_fields_params(
    mapping_trees: &mut HashSet<MappingTree>,
    field_to: &To<Params>,
    field: &field::Field,
) {
    if field_to.params.destination.is_none() {
        return;
    }
    let updated_mapping_trees = mapping_trees
        .iter()
        .filter(|&mapping_tree| {
            field_to.params.destination.is_none()
                || &mapping_tree.destination == field_to.params.destination.as_ref().unwrap()
        })
        .map(|mapping_tree| {
            let mut mapping_tree = mapping_tree.clone();
            let updated_fields = mapping_tree
                .mapping_fields
                .iter()
                .filter(|f| f.member == field.member)
                .map(|f| MappingField {
                    field: field_to.params.field.clone(),
                    ..f.clone()
                })
                .collect::<Vec<MappingField>>();
            for mapping_field in updated_fields {
                mapping_tree.mapping_fields.replace(mapping_field);
            }
            mapping_tree
        })
        .collect::<Vec<MappingTree>>();
    for mapping_tree in updated_mapping_trees {
        mapping_trees.replace(mapping_tree);
    }
}

fn add_mapping_field_for_additive_mapping_trees(
    mapping_trees: &mut HashSet<MappingTree>,
    value: &Struct,
    field_to: &To<Params>,
    field_strategy: &MappingStrategy,
    field: &field::Field,
) {
    if field_to.params.destination.is_none() {
        return;
    }
    let field_dest = field_to.params.destination.as_ref().unwrap();
    let mut mapping_tree = mapping_trees
        .get_or_insert(MappingTree::new(
            value.ident.clone(),
            field_dest.clone(),
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
    field_to: &To<Params>,
    field: &field::Field,
) {
    let mapping_trees_with_excluded_field_removed = mapping_trees
        .iter()
        .filter(|&mapping_tree| {
            field_to.params.destination.is_none()
                || &mapping_tree.destination == field_to.params.destination.as_ref().unwrap()
        })
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
