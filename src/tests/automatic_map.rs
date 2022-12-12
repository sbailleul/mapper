use mapper_api::Mapper;
use mapper_impl::Mapper;

#[cfg(test)]

#[derive(Mapper)]
#[to(Person)]
struct User{
    pub name: String
}
struct Person{
    pub name: String
}

pub fn derive_should_clone_all(){

}