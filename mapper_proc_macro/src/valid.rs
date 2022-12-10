
use quote::ToTokens;
use syn::{Error, Result, spanned::Spanned};
use thiserror::Error;

use crate::{ast::{Input, Struct},attr::{data_type:: {Attrs}}};
impl Input<'_> {
    pub(crate) fn validate(&self) -> Result<()> {
        match self {
            Input::Struct(input) => input.validate(),
        }
    }
}
#[derive(Error,Debug)]

pub enum StructError{
    #[error("Destination type for on field {0} is not referenced in destination types")]
    DestinationNotFound(String)
}
impl Struct<'_> {
    fn validate(&self) -> Result<()> {
        for field in &self.fields{
            let non_referenced_type =  field
            .attrs
            .to
            .iter()
            .find(|&to| self.attrs.to.destinations.iter().all(|dest| to.ty.get_ident() != dest.get_ident())); 
            if let Some(non_referenced_type) = non_referenced_type{
                return Err(Error::new(self.original.span(), StructError::DestinationNotFound(non_referenced_type.ty.get_ident().unwrap().to_string())))
            }
        }
        Ok(())
    }
}
