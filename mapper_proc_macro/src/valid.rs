use crate::{ast::{Input, Struct}, attr::Attrs};
use syn::Result;
impl Input<'_> {
    pub(crate) fn validate(&self) -> Result<()> {
        match self {
            Input::Struct(input) => input.validate(),
        }
    }
}
impl Struct<'_> {
    fn validate(&self) -> Result<()> {
        check_non_field_attrs(&self.attrs)?;
        Ok(())
    }
}

fn check_non_field_attrs(_attrs: &Attrs) -> Result<()> {
    Ok(())
}