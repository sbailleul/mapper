use syn::{parse::Parse, Attribute, Result};

#[derive(Clone, Debug)]
pub struct To<'a, T: Parse> {
    pub original: &'a Attribute,
    pub params: T
}
impl<'a, T: Parse> To<'a, T> {
    pub fn new(attr: &'a Attribute) -> Result<To<'a, T>> {
        let to = To {
            original: attr,
            params: attr.parse_args_with(T::parse)?,
        };
        Ok(to)
    }
}
