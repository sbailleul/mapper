use std::fmt::Debug;

use syn::punctuated::Punctuated;

use syn::spanned::Spanned;
use syn::{DeriveInput, Path};
use syn::{Attribute, Error, Result, Token};


#[derive(Debug)]
pub struct Attrs<'a> {
    pub to: To<'a>,
}

#[derive(Clone)]
pub struct To<'a> {
    pub original: &'a Attribute,
    pub destinations: Vec<Path>
}

impl Debug for To<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("To").finish()
    }
}

pub fn get(node: &DeriveInput) -> Result<Attrs> {
    for attr in &node.attrs {
        if attr.path.is_ident("to") {
            return parse_attribute( attr)
        }
    }
    Err(Error::new(node.span(), "Should contains exactly one to attribute"))
}

fn parse_attribute<'a>(attr: &'a Attribute) -> Result<Attrs> {
    
    let types_parser = Punctuated::<Path, Token![,]>::parse_terminated;
    let args = attr.parse_args_with(types_parser)?;
    let to = To {
        original: attr,
        destinations: args.into_iter().collect(),
    };
    Ok(Attrs{to})
}
