use syn::parse::Parse;

use syn::{Attribute, TypePath};


use self::params::Params;

use super::mapping_strategy::MappingStrategy;
pub mod params;

#[derive(Debug, Clone)]
pub struct Attrs<'a> {
    pub to: Vec<To<'a>>,
}

impl <'a> Attrs<'a>{
    pub fn has_to_exclude_field_for_destination(&self, destination: &TypePath)->bool{
        self.to.iter().any(|to| to.params.is_excluded_for_destination(destination))
    } 
    pub fn is_additive_mapping_for_destination_and_strategy(&self, destination: &TypePath, strategy: &MappingStrategy)->bool{
        self.to.iter().any(|to| to.params.is_additive_mapping_for_destination_and_strategy(destination, strategy))
    }
}

#[derive(Clone, Debug)]
pub struct To<'a> {
    pub original: &'a Attribute,
    pub params: Params
}



pub fn get(input: &syn::Field) -> syn::Result<Attrs> {
    let mut to_attributes = vec![];
    for attr in &input.attrs {
        if attr.path.is_ident("to") {
            to_attributes.push(To{params: attr.parse_args_with(Params::parse)?, original: attr});
        }
    }
    Ok(Attrs { to: to_attributes })
}
