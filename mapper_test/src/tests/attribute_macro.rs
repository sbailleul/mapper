use mapper_proc_macro::*;
use mapper::Mapper;

#[derive(Mapper, Clone)]
#[to(Vehicule)]
struct Animal {
    pub name: String,
}


struct Vehicule {
    pub name: String,
}

#[test]
fn test_macro() {
    let animal = Animal {
        name: "Toto".to_string(),
    };
    let vehicule:Vehicule = animal.to();
    assert_eq!(vehicule.name, animal.name)
}
