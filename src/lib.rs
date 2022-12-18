


pub use mapper_impl::*;
pub use mapper_api::*;

#[cfg(test)]
mod tests;


pub fn map_one_field_should_works() {
    fn map(val: &str)-> String{val.to_string()}
    
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        #[to(Person, with(into)=map)]
        name: String,
    }
    struct Person {
        name: String,
    }
    let user = User {
        name: "Marie".to_string(),
    };
    let person: Person = user.to();
    assert_eq!(person.name, user.name);
}