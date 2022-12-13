use mapper_api::Mapper;
use mapper_impl::Mapper;

#[cfg(test)]


#[test]
pub fn derive_should_clone_multiple_fields() {
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
pub fn derive_should_clone_one_field() {
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
pub fn derive_should_works_with_generic_types() {
    #[derive(Mapper)]
    #[to(Person<String>)]
    struct User {
        name: String,
    }
    struct Person<T> {
        name: T,
    }
    let user = User {
        name: "Marie".to_string(),
    };
    let person: Person<String> = user.to();
    assert_eq!(person.name, user.name);
}

