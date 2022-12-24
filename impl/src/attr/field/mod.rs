use syn::parse::Parse;

use syn::{Attribute, TypePath};


use self::params::Params;
pub mod params;

#[derive(Debug, Clone)]
pub struct Attrs<'a> {
    pub to: Vec<To<'a>>,
}

impl <'a> Attrs<'a>{
    pub fn has_to_exclude_field_for_destination(&self, destination: &TypePath)->bool{
        self.to.iter().any(|to| &to.params.destination == destination && to.params.exclude.1)
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
