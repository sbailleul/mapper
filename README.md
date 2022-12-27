# mapper

[![github]](https://github.com/sbailleul/mapper)&ensp;[![crates-io]](https://crates.io/crates/mapper)&ensp;[![docs-rs]](https://docs.rs/mapper)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[mapper-ci]: https://github.com/sbailleul/mapper/actions/workflows/rust.yml/badge.svg
This library provides a convenient derive macro for implementing [mapper_api::Mapper<T>] or [std::convert::Into] trait and generate mapping without boilerplate.

<br>

## Example
```rust
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

## Disclaimer
- Macro works only on C style struct like : struct MyStruct{field: u8}
- Mapper doesn't handle nested properties


## Default behavior
Default behavior is to take each field of annotated struct and clone those fields in the destination struct initializer by implementing [mapper_api::Mapper<T>] trait :
```rust
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
```rust
impl Mapper<Person> for User{
    fn to(&self)->Person{
        Person{name: self.name.clone()}
    }
}

```
## Mapping types
Two mapping types are available :
- Automatic, generate mapping for destinations specified in [to struct attributes](#to-struct-attribute),
all fields of the source are used for mapping if they are not explicitly excluded.
You can't use automatic mapping if additive mapping is already used for destination and strategy
- Additive, generate mapping for destinations specified in [to field attributes](#to-field-attribute), only annotated fields are mapped.
You can't use additive mapping if automatic mapping is already used for destination and strategy

## Mapping strategies
Two mapping strategies are available :
- mapper(default), map source to destination without consuming source, generate implementation of [mapper_api::Mapper<T>]
- into, map source to destination by consuming source  of [std::convert::Into]

## To struct attribute
Generate automatic mapping for specified strategies.
- You can set multiple to attribute by struct
- Specify one or multiple destination types in this attribute : ```#[to(Animal, Vehicle)]```
- Specify one or multiple mapping strategies in this attribute : ```#[to(Animal, strategy=into, strategy=mapper)]```

## To field attribute
Complete automatic mapping configuration set on parent struct or provide additive mapping or exclude field from any mappings
- You can set multiple to attribute by field
- This attribute is forbidden if you use only DestinationType


### DestinationType
Type of the mapping destination. Mandatory argument unless field is unconditionally excluded.
#### Generics
You can specify destination type with generics, these generics should be compatible with the fields of your src struct :
```rust
#[derive(Mapper)]
#[to(Person::<String, u8>)]
struct User {
    name: String,
    age: u8
}
struct Person<T, U> {
    name: T,
    age: U
}
```

### Strategy
Trigger additive mapping for mapping destination and specified strategy,

### Exclude
Optional parameter, specify if the field is excluded for mapping, there is 2 kind of exclusion.
- Unconditionally exclusion, exclude field of any kind of mapping e.g ```#[to(exclude)]```
- Exclusion for specific destination, determined for destination e.g ```#[to(Destination, exclude]```
⚠️ not works for additive mapping

### Field
Optional parameter, target the destination type field

### With
Optional parameter, provide a function to transform the annotated field to the destination field.
You can specify strategy used by with function as following : ```with(<strategy>)``` if you use with without specifying strategy : ```with``` mapper strategy will be used by default
Signature of the function should be in regards of used strategy  :
-  with(mapper) | with :
```rust
fn foo_mapping(val: &<src_field_type>)-><dst_field_type>
```
- with(into) :
```rust
fn foo_mapping(val: <src_field_type>)-><dst_field_type>
```

#### Generics
You can use generics in your function if the generic types constraint respect the source field type and destination field type :
```rust
    fn gen_func<T: Display>(val: &T)->String{
        val.to_string()
    }
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        #[to(Person, with=gen_func)]
        age: u16,
    }
    struct Person {
        age: String,
    }
```


License: MIT OR Apache-2.0
