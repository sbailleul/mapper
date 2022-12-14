use std::fmt::Display;

use mapper_api::Mapper;
use mapper_impl::Mapper;

#[cfg(test)]
#[test]
pub fn map_with_func_should_works() {
    fn with_fun(val: &u16) -> String {
        val.to_string()
    }
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        #[to(Person, with=with_fun)]
        age: u16,
    }
    struct Person {
        age: String,
    }
    let user = User { age: 123 };
    let person: Person = user.to();
    assert_eq!("123", person.age);
}

#[test]
pub fn map_with_generic_func_should_works() {
    fn gen_func<T: Display>(val: &T) -> String {
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
    let user = User { age: 123 };
    let person: Person = user.to();
    assert_eq!("123", person.age);
}

#[test]
pub fn map_generic_destination_type_with_func_should_works() {
    fn map_u16_to_string(val: &u16) -> String {
        val.to_string()
    }
    #[derive(Mapper)]
    #[to(Person::<String>)]
    struct User {
        #[to(Person::<String>, with=map_u16_to_string)]
        age: u16,
    }
    struct Person<T> {
        age: T,
    }
    let user = User { age: 123 };
    let person: Person<String> = user.to();
    assert_eq!("123", person.age);
}

#[test]
pub fn exclude_field_should_works() {
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        age: u16,
        #[to(Person, exclude)]
        name: String,
    }
    struct Person {
        age: u16,
    }
    let user = User {
        age: 123,
        name: "Marie".to_owned(),
    };
    let person: Person = user.to();
    assert_eq!(123, person.age);
}

#[test]
pub fn use_with_strategy_should_works() {
    fn map_into(val: String) -> String {
        val
    }
    fn map_mapper(val: &str) -> String {
        val.to_string()
    }
    #[derive(Mapper)]
    #[to(Person, strategy=into, strategy=mapper)]
    struct User {
        #[to(Person, with(into)=map_into, with(mapper)=map_mapper)]
        name: String,
    }
    struct Person {
        name: String,
    }
    let user = User {
        name: "Marie".to_owned(),
    };
    let person_mapper: Person = user.to();
    let person_into: Person = user.into();

    assert_eq!("Marie", person_into.name);
    assert_eq!("Marie", person_mapper.name);
}

#[test]
pub fn use_with_strategy_should_works_for_for_additive_and_automatic_mapping() {
    fn map_into(val: String) -> String {
        val.to_uppercase()
    }
    fn map_mapper(val: &str) -> String {
        val.to_lowercase()
    }
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        #[to(Person, with(into)=map_into, with(mapper)=map_mapper, strategy=into)]
        name: String,
    }
    struct Person {
        name: String,
    }
    let user = User {
        name: "Marie".to_owned(),
    };
    let person_mapper: Person = user.to();
    let person_into: Person = user.into();

    assert_eq!("MARIE", person_into.name);
    assert_eq!("marie", person_mapper.name);
}

#[test]
pub fn exclude_field_without_destination_should_never_be_mapped() {
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        #[to(exclude)]
        name: String,
        age: u16,
    }
    struct Person {
        age: u16,
    }
    let user = User {
        name: "Marie".to_owned(),
        age: 30,
    };
    let person: Person = user.to();

    assert_eq!(30, person.age);
}

#[test]
pub fn additive_mapping_use_all_strategies() {
    #[derive(Mapper)]
    struct User {
        #[to(Person, strategy=all)]
        age: u16,
    }
    struct Person {
        age: u16,
    }
    let user = User { age: 30 };
    let person_mapper: Person = user.to();
    let person_into: Person = user.into();
    assert_eq!(30, person_into.age);
    assert_eq!(30, person_mapper.age);
}

#[test]
pub fn field_mapping_should_works() {
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        #[to(Person, field=_age)]
        age: u16,
    }
    struct Person {
        _age: u16,
    }
    let user = User { age: 30 };
    let person_mapper: Person = user.to();
    assert_eq!(30, person_mapper._age);
}
