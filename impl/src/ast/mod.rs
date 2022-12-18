use std::fmt::Debug;

use proc_macro2::{Span, TokenStream};

use syn::{DeriveInput, Ident, Generics, Member, Type, Result, Data, DataStruct, Error, Fields, Index, spanned::Spanned, Path, TypePath};

use self::data_type::Struct;


pub mod field;
pub mod data_type;

#[derive(Debug)]
pub enum Input<'a> {
    Struct(Struct<'a>)
}


impl Debug for Struct<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
       .finish()
    }
}


impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            _ => Err(Error::new_spanned(node, "Only structs are supported")),
        }
    }
}



// pub struct MappingField{
//     pub with: 
// }
// pub struct MappingTree{
//     pub destination: TypePath,
//     pub 
// }