
/*! [![github]](https://github.com/sbailleul/mapper)&ensp;[![crates-io]](https://crates.io/crates/mapper)&ensp;[![docs-rs]](https://docs.rs/mapper)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
This library provides a convenient derive macro for implementing [mapper_api::Mapper<T>] trait and generate mapping without boilerplate.

<br>

# Example
```ignore
# use std::io;
use mapper::Mapper;

fn map_account_id(account_id: &u16) -> String{
    account_id.to_string()
}

#[derive(Mapper)]
#[to(Person)]
struct User{
    #[to(Person, field=_name)]
    pub name: String,
    #[to(Person, with=map_account_id)]
    pub account_id: u16,
    pub age: u8
}
struct Person{
    pub _name: String,
    pub account_id: String,
    pub age: u8
}
```

<br>

# Disclaimer
- Macro works only on C style struct like : struct MyStruct{field: u8}
- Mapper doesn't handle generics
- Mapper doesn't handle nested properties


# Default behavior
Default behavior is to take each field of annotated struct and clone 
those fields in the destination struct initializer :  
```ignore
#[derive(Mapper)]
#[to(Person)]
struct User{
    pub name: String
}
struct Person{
    pub name: String
}
```  
Generate 🔄 : 
```ignore 
impl Mapper<Person> for User{
    fn to(&self)->Person{
        Person{name: self.name.clone()}
    }
}
```

# To struct attribute
- You can specify multiple destination types in this attribute : ```#[to(Animal, Vehicle)]```

# To field attribute
- You can put multiple to attribute by field
- Syntax of this attribute : ```#[to(<DestinationType>, field=<destination_field>, with=<transformation_function>)]```
- This attribute is forbidden if you use only DestinationType 

## DestinationType 
This parameter is mandatory and have to be present in the [To struct attribute](#to-struct-attribute)

## Field 
Optional parameter, target the destination type field

## With 
Optional parameter, provide a function to transform the annotated field to the destination field
*/

pub use mapper_impl::*;
pub use mapper_api::*;

#[cfg(test)]
mod tests;
