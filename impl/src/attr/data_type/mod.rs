use std::collections::HashSet;

use syn::DeriveInput;
use syn::{Error, Result};

use self::params::Params;

use super::aggregated_to::AggregatedTo;
use super::attrs::Attrs;

use super::to::To;
pub mod params;

pub fn get(node: &DeriveInput) -> Result<Attrs<To<Params>>> {
    let mut aggregated_to = AggregatedTo::new();
    for attr in &node.attrs {
        if attr.path.is_ident("to") {
            let to = To::<Params>::new(attr)?;
            for strategy in &to.params.strategies {
                let registered_destinations = aggregated_to
                    .destinations_by_strategy
                    .entry(strategy.clone())
                    .or_insert_with(HashSet::new);
                let to_destinations = to.params.destinations.clone();
                let common_destinations = to_destinations.intersection(registered_destinations);
                if common_destinations.clone().count() > 0 {
                    return Err(
                        Error::new_spanned(attr,
                            format!("You cannot specify multiple time same destination for a given strategy, strategy ({}), destinations ({})"
                            ,strategy
                            ,common_destinations.map(|dest|dest.path.get_ident().unwrap().to_string()).collect::<Vec<String>>().join(",")
                    )
                        ));
                }
                registered_destinations.extend(to_destinations);
            }
            aggregated_to.to_items.push(to);
        }
    }
    Ok(Attrs { to: aggregated_to })
}
