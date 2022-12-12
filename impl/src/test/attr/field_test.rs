use syn::{Path, AttributeArgs};
use syn::parse::ParseStream;
use syn::token::Struct;

use crate::attr::field::To;
use crate::test::{parse_str_err, parse_str_success};

use super::ToCreationError;
#[cfg(test)]

#[test]
fn should_create_field_configured_to() {
    let res = parse_str_success::<To>(r#"Vehicule, field=name"#);
    assert_eq!("Vehicule", res.ty.get_ident().unwrap().to_string());
    assert_eq!("name", res.field.unwrap().get_ident().unwrap().to_string());
}

#[test]
fn should_create_with_configured_to() {
    let res = parse_str_success::<To>(r#"Vehicule, with=mapfunc"#);
    assert_eq!("Vehicule", res.ty.get_ident().unwrap().to_string());
    assert_eq!("mapfunc", res.with.unwrap().get_ident().unwrap().to_string());
}



#[test]
fn should_create_fully_configured_to() {
    let res = parse_str_success::<To>(r#"Vehicule, with=mapfunc, field=name"#);
    assert_eq!("Vehicule", res.ty.get_ident().unwrap().to_string());
    assert_eq!("mapfunc", res.with.unwrap().get_ident().unwrap().to_string());
    assert_eq!("field", res.field.unwrap().get_ident().unwrap().to_string());
}

#[test]
fn should_failed_if_type_is_not_specified() {
    let path = parse_str_success::<Path>("field_value");
    let with = parse_str_success::<Path>("with_value");
    let res = To::new(None, Some(path), Some(with));
    let err = res.err().unwrap();
    assert!(matches!(ToCreationError::MissingDestination, err));
}

#[test]
fn should_failed_if_config_is_not_specified() {
    let res = To::new(None, None, None);
    let err = res.err().unwrap();
    assert!(matches!(ToCreationError::MissingConfigField, err));
}
