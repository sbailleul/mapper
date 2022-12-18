use std::{fmt::Debug};
use syn::{DeriveInput, Result, Data, Error};



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



// #[derive(Hash, PartialEq, Eq, Debug)]
// pub struct MappingField{
//     pub ty: TypePath,
//     pub member: Member,
//     pub field: Option<Path>,
//     pub with: Option<Path>
// }
// pub struct MappingTree{
//     pub ident: Ident,
//     pub destination: TypePath,
//     pub strategy: MappingStrategy,
//     pub mapping_fields: HashSet<MappingField>
// }

// impl TryFrom<Struct<'_>> for Vec<MappingTree>{
//     type Error = syn::Error;

//     fn try_from(value: Struct<'_>) -> Result<Self> {
//         let mapping_trees = vec![];
        

//     }
// }


