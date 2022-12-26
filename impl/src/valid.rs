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
            for field_to in &field.attrs.to.to_items {
                if field_to.params.destination.is_none() {
                    break;
                }
                let field_dest = field_to.params.destination.as_ref().unwrap();
                for field_strategy in &field_to.params.strategies {
                    if let Some(struct_destinations) =
                        self.attrs.to.destinations_by_strategy.get(field_strategy)
                    {
                        if struct_destinations.contains(field_dest) {
                            return Err(Error::new_spanned(
                                &field_strategy.0,
                                format!(
                                    "Additive mapping not works for destination ({}) and strategy ({}) because it's already used in automatic mapping", 
                                    field_dest.path.get_ident().unwrap(),
                                    field_strategy
                            ),
                            ));
                        }
                    }
                }
                if field_to.params.exclude.1 && !self.attrs.to.destinations().contains(field_dest) {
                    return Err(Error::new_spanned(
                        &field_to.params.exclude.0,
                         format!(
                            "Cannot exclude a field for a destination ({}) not referenced in automatic mapping",
                             field_dest.path.get_ident().unwrap())
                            ));
                }

                for field_with in &field_to.params.with {
                    if !self.has_strategy_for_destination(field_dest, &field_with.1) {
                        return Err(Error::new_spanned(
                            field_with.0.clone(),
                             format!(
                                "There is no destination and strategy matching with strategy ({}) and destination ({})",
                                field_with.1,
                                field_dest.path.get_ident().unwrap()
                            )));
                    }
                }
            }
        }
        Ok(())
    }
}
