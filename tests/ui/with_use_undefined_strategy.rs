use mapper_impl::Mapper;

fn with_test(val: &String)->String{
    val
}

#[derive(Mapper)]
#[to(Person,strategy=into)]
pub struct User{
    #[to(Person, with=with_test)]
    pub name: String
}

pub struct Person{
    pub name: String
}

fn main(){}