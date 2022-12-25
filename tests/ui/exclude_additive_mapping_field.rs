use mapper_impl::Mapper;

#[derive(Mapper)]
pub struct User{
    #[to(Person, exclude, strategy=into)]
    pub name: String
}

pub struct Person{
    pub name: String
}

fn main(){}