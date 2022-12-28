use std::collections::HashSet;

use syn::{Error, Path, TypePath};

use self::params::Params;

use super::mapping_strategy::MappingStrategy;

pub mod params;

use super::aggregated_to::AggregatedTo;
use super::attrs::Attrs;
use super::to::To;

impl To<'_, Params> {
    pub fn get_with_by_strategy(&self, strategy: &MappingStrategy) -> Option<Path> {
        let with = self.params.with.iter().find(|&w| &w.1 == strategy);
        Option::flatten(with.map(|w| w.0.clone()))
    }
    pub fn is_excluded_for_destination(&self, destination: &TypePath) -> bool {
        self.params.exclude.1
            && if let Some(dest) = &self.params.destination {
                dest == destination
            } else {
                true
            }
    }
    pub fn is_additive_mapping_for_destination_and_strategy(
        &self,
        destination: &TypePath,
        strategy: &MappingStrategy,
    ) -> bool {
        if let Some(dest) = &self.params.destination {
            dest == destination
                && self
                    .params
                    .strategies
                    .iter()
                    .any(|self_strategy| &self_strategy.1 == strategy)
        } else {
            false
        }
    }
    pub fn is_additive_mapping_for_destination(&self, destination: &TypePath) -> bool {
        if let Some(dest) = &self.params.destination {
            dest == destination && !self.params.strategies.is_empty()
        } else {
            false
        }
    }
}

pub fn get(input: &syn::Field) -> syn::Result<Attrs<To<Params>>> {
    let mut aggregated_to = AggregatedTo::new();
    for attr in &input.attrs {
        if attr.path.is_ident("to") {
            let to = To::<Params>::new(attr)?;
            for strategy in &to.params.strategies {
                let registered_destinations = aggregated_to
                    .destinations_by_strategy
                    .entry(strategy.clone())
                    .or_insert_with(HashSet::new);
                if let Some(to_destination) = &to.params.destination {
                    if let Some(destination) =
                        registered_destinations.replace(to_destination.clone())
                    {
                        return Err(
                            Error::new_spanned(attr,
                                format!("You cannot specify multiple time same destination for a given strategy, strategy ({}), destination ({})"
                                ,strategy
                                ,destination.path.get_ident().unwrap()
                        )));
                    }
                }
            }
            aggregated_to.to_items.push(to);
        }
    }
    Ok(Attrs { to: aggregated_to })
}
