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
        #[to(Person::<u16>, with=map_u16_to_string)]
        age: u16,
    }
    struct Person<T> {
        age: T,
    }
    let user = User { age: 123 };
    let person: Person<String> = user.to();
    assert_eq!("123", person.age);
}

