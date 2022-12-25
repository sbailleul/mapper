use std::collections::{HashMap, HashSet};

use syn::{Path, TypePath};

use super::{mapping_strategy::MappingStrategy, spanned_item::SpannedItem};

#[derive(Debug, Clone)]
pub struct AggregatedTo<T> {
    pub destinations_by_strategy: HashMap<SpannedItem<Path, MappingStrategy>, HashSet<TypePath>>,
    pub to_items: Vec<T>,
}

impl<T> AggregatedTo<T> {
    pub fn new() -> Self {
        AggregatedTo {
            destinations_by_strategy: HashMap::new(),
            to_items: vec![],
        }
    }
    pub fn destinations(&self) -> HashSet<TypePath> {
        self.destinations_by_strategy
            .values()
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }
    pub fn has_destination_for_strategy(
        &self,
        destination: &TypePath,
        strategy: &MappingStrategy,
    ) -> bool {
        if let Some(destinations) = self
            .destinations_by_strategy
            .get(&SpannedItem::new_empty(strategy.clone()))
        {
            destinations.contains(destination)
        } else {
            false
        }
    }
}
