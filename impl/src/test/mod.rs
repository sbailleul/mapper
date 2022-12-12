use syn::{parse, Error};
mod attr;

pub fn parse_str_success<T: parse::Parse>(input: &str) -> T{
    let stream = input.parse().unwrap();
    syn::parse2::<T>(stream).unwrap()
}

pub fn parse_str_err<T: parse::Parse>(input: &str) -> Error{
    let stream = input.parse().unwrap();
    syn::parse2::<T>(stream).err().unwrap()
}