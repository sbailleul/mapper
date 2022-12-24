


use std::fmt::Display;

pub use mapper_impl::*;
pub use mapper_api::*;

#[cfg(test)]
mod tests;





fn use_with_strategy_should__for_additive_mapping(){
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
