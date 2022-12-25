use mapper_impl::Mapper;

#[derive(Mapper)]
#[to(Person)]
pub struct User{
    #[to(Person,  strategy=mapper)]
    pub name: String
}

pub struct Person{
    pub name: String
}

fn main(){}