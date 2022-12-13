use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::vec;

use syn::parse::Parse;
use syn::punctuated::Punctuated;

use syn::spanned::Spanned;
use syn::{Attribute, Error, Result, Token};
use syn::{DeriveInput, Expr, Path};

#[derive(Debug)]
pub struct Attrs<'a> {
    pub to: To<'a>,
}

#[derive(Clone, Debug)]
pub struct To<'a> {
    pub original: &'a Attribute,
    pub params: Params,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Params {
    pub destinations: HashSet<Path>,
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut destinations = HashSet::new();
        let args = Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)
            .expect("Attribute shouldn't be empty");
        for arg in args {
            match arg {
                Expr::Path(path) => {
                    destinations.insert(path.path);
                }
                _ => (),
            }
        }
        Ok(Params { destinations })
    }
}

pub fn get(node: &DeriveInput) -> Result<Attrs> {
    for attr in &node.attrs {
        if attr.path.is_ident("to") {
            return Ok(Attrs {
                to: To {
                    original: attr,
                    params: attr.parse_args_with(Params::parse)?,
                },
            });
        }
    }
    Err(Error::new(
        node.span(),
        "Should contains exactly one to attribute",
    ))
}
