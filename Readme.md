# Mapper 

This library provides a way to create simple mapping between types 



```rust
struct User{
    pub name: String;
    pub _age: i32;
}

#[derive(Mapper)]
#[to(User)]
struct Person{
    pub name: String
    #[to(User::_age)]
    pub age: i32
}



```