use syn::{Error, Result};

use crate::ast::{data_type::Struct, Input};
impl Input<'_> {
    pub(crate) fn validate(&self) -> Result<()> {
        match self {
            Input::Struct(input) => input.validate(),
        }
    }
}

impl Struct<'_> {
    fn validate(&self) -> Result<()> {
        for field in &self.fields {
            for field_to in &field.attrs.to {
                for field_strategy in &field_to.params.strategies {
                    if let Some(struct_destinations) =
                        self.attrs.to.destinations_by_strategy.get(field_strategy)
                    {
                        if struct_destinations.contains(&field_to.params.destination) {
                            return Err(Error::new_spanned(
                                &field_strategy.0,
                                format!(
                                    "Additive mapping not works for destination {} and strategy because it's already used in automatic mapping {}", 
                                    field_to.params.destination.path.get_ident().unwrap().to_string(),
                                    field_strategy
                            ),
                            ));
                        }
                    }
                }
                if field_to.params.exclude.1
                    && !self
                        .attrs
                        .to
                        .destinations()
                        .contains(&field_to.params.destination)
                {
                    return Err(Error::new_spanned(
                        &field_to.params.exclude.0,
                         format!(
                            "Cannot exclude a field for a destination {} not referenced in automatic mapping",
                             field_to.params.destination.path.get_ident().unwrap().to_string()) 
                            ));
                }
            }
        }
        Ok(())
    }
}
