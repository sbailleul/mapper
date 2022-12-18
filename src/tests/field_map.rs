use std::fmt::Display;

use mapper_api::Mapper;
use mapper_impl::Mapper;

#[cfg(test)]


#[test]
pub fn map_with_func_should_works() {
    fn with_fun(val: &u16)->String{
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
    let user = User {
        age: 123
    };
    let person: Person = user.to();
    assert_eq!("123", person.age);
}

#[test]
pub fn map_with_generic_func_should_works() {
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
    let user = User {
        age: 123
    };
    let person: Person = user.to();
    assert_eq!("123", person.age);
}

#[test]
pub fn map_generic_destination_type_with_func_should_works(){
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
pub fn exclude_field_should_works(){
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        age: u16,
        #[to(Person, exclude)]
        name: String
    }
    struct Person {
        age: u16,
    }
    let user = User { age: 123, name: "Marie".to_owned() };
    let person: Person = user.to();
    assert_eq!(123, person.age);
}

#[test]
pub fn use_with_strategy_should_works(){
    fn map_into(val: String) -> String{
        val
    }
    fn map_mapper(val: &str) -> String{
        val.to_string()
    }
    #[derive(Mapper)]
    #[to(Person, strategy=into, strategy=mapper)]
    struct User {
        #[to(Person, with(into)=map_into, with(mapper)=map_mapper)]
        name: String
    }
    struct Person {
        name: String,
    }
    let user = User {  name: "Marie".to_owned() };
    let person_mapper: Person = user.to();
    let person_into: Person = user.into();
    
    assert_eq!("Marie", person_into.name);
    assert_eq!("Marie", person_mapper.name);
}

