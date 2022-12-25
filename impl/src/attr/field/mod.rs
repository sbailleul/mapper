use std::collections::{HashMap, HashSet};

use syn::parse::Parse;

use syn::{Attribute, TypePath, Path, Error};


use self::params::Params;

use super::mapping_strategy::MappingStrategy;
use super::spanned_item::SpannedItem;
pub mod params;

use super::aggregated_to::AggregatedTo ;
use super::attr::Attrs ;
use super::to::To;




impl To<'_, Params>{
    pub fn get_with_by_strategy(&self, strategy: &MappingStrategy) -> Option<Path> {
        let with = self.params.with.iter().find(|&w| &w.1 == strategy);
        Option::flatten(with.map(|w| w.0.clone()))
    }
    pub fn is_excluded_for_destination(&self, destination: &TypePath)->bool{
        &self.params.destination == destination && self.params.exclude.1
    } 
    pub fn is_additive_mapping_for_destination_and_strategy(&self, destination: &TypePath, strategy: &MappingStrategy)->bool{
        &self.params.destination == destination && self.params.strategies.iter().any(|self_strategy|  &self_strategy.1 == strategy)
    } 
    pub fn is_additive_mapping_for_destination(&self, destination: &TypePath)->bool{
        &self.params.destination == destination && !self.params.strategies.is_empty()
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
                    .or_insert(HashSet::new());
                let to_destination = to.params.destination.clone();
                if let Some(destination) = registered_destinations.replace(to_destination){
                    return Err(
                        Error::new_spanned(attr, 
                            format!("You cannot specify multiple time same destination for a given strategy, strategy ({}), destination ({})"
                            ,strategy
                            ,destination.path.get_ident().unwrap().to_string()
                    )))
                }
            } 
            aggregated_to.to_items.push(to);
        }
    }
    Ok(Attrs { to: aggregated_to })
}
