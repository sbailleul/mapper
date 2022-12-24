use syn::{spanned::Spanned, Error, Result};
use thiserror::Error;

use crate::ast::{data_type::Struct, Input};
impl Input<'_> {
    pub(crate) fn validate(&self) -> Result<()> {
        match self {
            Input::Struct(input) => input.validate(),
        }
    }
}
#[derive(Error, Debug)]

pub enum StructError {
    #[error("Destination type for on field {0} is not referenced in destination types")]
    DestinationNotFound(String),
}
impl Struct<'_> {
    fn validate(&self) -> Result<()> {
        for field in &self.fields {
            for field_to in &field.attrs.to {
                for field_strategy in &field_to.params.strategies {
                    if let Some(struct_destinations) =
                        self.attrs.to.destinations_by_strategy.get(&*field_strategy)
                    {
                        if struct_destinations.contains(&field_to.params.destination) {
                            return Err(Error::new_spanned(
                                &field_strategy.0,
                                format!("To struct attribute already specify strategy {}", field_strategy),
                            ));
                        }
                    } 
                }
                if field_to.params.exclude.1 && !self.attrs.to.destinations().contains(&field_to.params.destination){
                    return Err(Error::new_spanned(&field_to.params.exclude.0, "Cannot exclude a field for a destination not referenced in automatic mapping"));
                }
            }
        }
        Ok(())
    }
}
