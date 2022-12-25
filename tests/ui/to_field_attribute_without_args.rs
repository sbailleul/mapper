use mapper_impl::Mapper;

#[derive(Mapper)]
pub struct User{
    #[to()]
    pub name: String
}

pub struct Person{
    pub name: String
}

fn main(){}