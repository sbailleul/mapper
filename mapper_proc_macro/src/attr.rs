use std::fmt::Debug;

use syn::punctuated::Punctuated;

use syn::TypePath;
use syn::{Attribute, Error, Result, Token};


#[derive(Debug)]
pub struct Attrs<'a> {
    pub to: Option<To<'a>>,
}

#[derive(Clone)]
pub struct To<'a> {
    pub original: &'a Attribute,
    pub destinations: Vec<TypePath>
}

impl Debug for To<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("To").finish()
    }
}

pub fn get(input: &[Attribute]) -> Result<Attrs> {
    let mut attrs = Attrs { to: None };

    for attr in input {
        if attr.path.is_ident("to") {
            parse_to_attribute(&mut attrs, attr)?;
        }
    }

    Ok(attrs)
}

fn parse_to_attribute<'a>(attrs: &mut Attrs<'a>, attr: &'a Attribute) -> Result<()> {
    
    let types_parser = Punctuated::<TypePath, Token![::]>::parse_terminated;
    let args = attr.parse_args_with(types_parser)?;

    let to = To {
        original: attr,
        destinations: args.into_iter().collect(),
    };
    if attrs.to.is_some() {
        return Err(Error::new_spanned(
            attr,
            "only one #[to(...)] attribute is allowed",
        ));
    }
    attrs.to = Some(to);
    Ok(())
}
