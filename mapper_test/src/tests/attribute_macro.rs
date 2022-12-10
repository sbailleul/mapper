use mapper_proc_macro::*;
use mapper::Mapper;

#[derive(Mapper, Clone)]
#[to(Vehicule)]
struct Animal {
    #[to(Vehicule, field=_name, with=map)]
    pub name: String,
}

struct Vehicule {
    pub _name: String,
}
fn map(txt: String)-> String{
    txt.to_uppercase()
}

#[test]
fn test_macro() {
    let animal = Animal {
        name: "Toto".to_string(),
    };
    let vehicule:Vehicule = animal.to();
    assert_eq!(vehicule._name, "TOTO")
}

