use mapper_api::Mapper;
use mapper_impl::Mapper;

#[cfg(test)]
#[test]
pub fn map_multiple_fields_should_works() {
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
        name: String,
        account_id: String,
    }
    struct Person {
        name: String,
        account_id: String,
    }
    let user = User {
        account_id: "ID-123".to_owned(),
        name: "Marie".to_string(),
    };
    let person: Person = user.to();
    assert_eq!(person.name, user.name);
    assert_eq!(person.account_id, user.account_id);
}

#[test]
pub fn map_one_field_should_works() {
    #[derive(Mapper)]
    #[to(Person)]
    struct User {
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
#[test]
pub fn map_field_with_generic_destination_type() {

    #[derive(Mapper)]
    #[to(Person::<String>)]
    struct User {
        name: String,
    }
    struct Person<T> {
        name: T,
    }
    let user = User { name: "Marie".to_owned() };
    let person: Person<String> = user.to();
    assert_eq!("Marie", person.name);
}

#[test]
pub fn map_field_with_generics_destination_type() {

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
    let user = User { name: "Marie".to_owned(), age: 35 };
    let person: Person<String, u8> = user.to();
    assert_eq!("Marie", person.name);
}
