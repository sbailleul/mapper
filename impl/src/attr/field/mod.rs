use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{Error, Path, Attribute, Type, TypePath};
use syn::{Expr, Token};

use self::params::Params;
pub mod params;

#[derive(Debug)]
pub struct Attrs<'a> {
    pub to: Vec<To<'a>>,
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
            to_attributes.push(To{params: attr.parse_args_with(Params::parse)?, original: attr });
        }
    }
    Ok(Attrs { to: to_attributes })
}
