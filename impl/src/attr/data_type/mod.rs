use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::vec;

use syn::parse::Parse;
use syn::punctuated::Punctuated;

use syn::spanned::Spanned;
use syn::{Attribute, Error, Generics, Pat, Result, Token, Type, TypePath};
use syn::{DeriveInput, Expr, Path};

use self::params::Params;

use super::mapping_strategy::MappingStrategy;
pub mod params;

#[derive(Debug)]
pub struct Attrs<'a> {
    pub to: AggregatedTo<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct To<'a> {
    pub original: &'a Attribute,
    pub params: Params,
}
impl<'a> To<'a> {
    fn new(attr: &'a Attribute) -> Result<To<'a>> {
        let to = To {
            original: attr,
            params: attr.parse_args_with(Params::parse)?,
        };
        Ok(to)
    }
}

#[derive(Debug)]
pub struct AggregatedTo<'a> {
    pub destinations_by_strategy: HashMap<Rc<MappingStrategy>, HashSet<Rc<TypePath>>>,
    pub to: Vec<To<'a>>
}

impl<'a> AggregatedTo<'a> {
    pub fn new() -> Self {
        AggregatedTo {
            destinations_by_strategy: HashMap::new(),
            to: vec![]
        }
    }
    pub fn destinations(&self) -> HashSet<Rc< TypePath>>{
        self.destinations_by_strategy.values().into_iter().flatten().map(|dest| dest.clone()).collect()
    }
}
pub fn get(node: &DeriveInput) -> Result<Attrs> {
    let mut aggregated_to = AggregatedTo::new();
    for attr in &node.attrs {
        if attr.path.is_ident("to") {
            let to = To::new(attr)?;
            for strategy in &to.params.strategies {
                let registered_destinations = aggregated_to
                    .destinations_by_strategy
                    .entry(strategy.clone())
                    .or_insert(HashSet::new());
                let to_destinations = to.params.destinations.clone();
                let common_destinations = to_destinations.intersection(registered_destinations);
                if common_destinations.clone().count() > 0{
                    return Err(
                        Error::new_spanned(attr, 
                            format!("You cannot specify multiple time same destination for a given strategy, strategy: {:?}, destinations : {:?}",strategy, common_destinations)
                        ))
                }
                registered_destinations.extend(to_destinations);
            }
            aggregated_to.to.push(to);
        }
    }
    Ok(Attrs { to: aggregated_to })
}
