use std::collections::HashSet;
use std::hash::Hash;

use proc_macro2::Ident;
use syn::{Member, TypePath};

use crate::attr::mapping_strategy::MappingStrategy;

use super::mapping_field::MappingField;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MappingType {
    Additive,
    Automatic,
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
    pub fn remove_mapping_fields_by_member(&mut self, member: &Member) {
        let fields = self
            .mapping_fields
            .iter()
            .filter(|&field| &field.member == member)
            .cloned()
            .collect::<Vec<MappingField>>();
        for field in fields {
            self.mapping_fields.remove(&field);
        }
    }
    pub fn has_mapping_type(&self, mapping_type: &MappingType) -> bool {
        if let Some(mapping_tree_type) = &self.mapping_type {
            mapping_tree_type == mapping_type
        } else {
            false
        }
    }
}
