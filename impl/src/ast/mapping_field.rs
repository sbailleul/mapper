use crate::attr::mapping_strategy::MappingStrategy;
use std::hash::Hash;
use syn::{Member, Path, Type};

#[derive(Eq, Debug, Clone)]
pub struct MappingField {
    pub ty: Type,
    pub strategy: MappingStrategy,
    pub member: Member,
    pub field: Option<Path>,
    pub with: Option<Path>,
}

impl Hash for MappingField {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.strategy.hash(state);
        self.member.hash(state);
    }
}
impl PartialEq for MappingField {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.strategy == other.strategy && self.member == other.member
    }
}
