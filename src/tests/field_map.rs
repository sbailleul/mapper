use std::fmt::Display;

use mapper_api::Mapper;
use mapper_impl::Mapper;

#[cfg(test)]


#[test]
pub fn to_should_user_with_func() {
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
pub fn to_should_user_with_gen_func() {
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

