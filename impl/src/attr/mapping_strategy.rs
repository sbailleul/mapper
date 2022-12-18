use std::{collections::HashSet, fmt::Display, rc::Rc};

use syn::{spanned::Spanned, Error, Path, Result as SynResult};
use thiserror::Error;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum MappingStrategy {
    Into,
    Mapper,
}

impl Display for MappingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingStrategy::Into => write!(f, "into"),
            MappingStrategy::Mapper => write!(f, "mapper"),
        }
    }
}

#[derive(Error, Debug)]
#[error("Invalid strategy {0}, available values : [into, mapper]")]
pub struct MappingStrategyParseError(String);
impl Default for MappingStrategy {
    fn default() -> Self {
        MappingStrategy::Mapper
    }
}

impl TryFrom<&str> for MappingStrategy {
    type Error = MappingStrategyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "into" => Ok(MappingStrategy::Into),
            "mapper" => Ok(MappingStrategy::Mapper),
            _ => Err(MappingStrategyParseError(value.to_string())),
        }
    }
}
pub const MAX_STRATEGIES_BY_ATTRIBUTE: usize = 2;

pub fn parse_strategy(
    path: &Path,
    strategies: &HashSet<Rc<MappingStrategy>>,
) -> SynResult<MappingStrategy> {
    if strategies.len() >= MAX_STRATEGIES_BY_ATTRIBUTE {
        Err(Error::new(path.span(), "Only two strategies are available"))
    } else {
        let ident = path
            .get_ident()
            .ok_or(Error::new(path.span(), "Invalid strategy"))?;
        MappingStrategy::try_from(ident.to_string().as_ref())
            .map_err(|e| Error::new(path.span(), e))
    }
}
