use std::{collections::HashSet, fmt::Display};

use syn::{Error, Path, Result as SynResult};
use thiserror::Error;

use super::spanned_item::SpannedItem;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum MappingStrategy {
    Into,
    Mapper,
}

impl MappingStrategy {
    pub fn str_to_hash_set(
        val: &str,
    ) -> Result<HashSet<MappingStrategy>, MappingStrategyParseError> {
        match val {
            "all" => Ok([MappingStrategy::Into, MappingStrategy::Mapper]
                .into_iter()
                .collect()),
            _val => {
                let mut set = HashSet::new();
                set.insert(MappingStrategy::try_from(_val)?);
                Ok(set)
            }
        }
    }
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
    strategies: &HashSet<SpannedItem<Path, MappingStrategy>>,
) -> SynResult<HashSet<SpannedItem<Path, MappingStrategy>>> {
    if strategies.len() >= MAX_STRATEGIES_BY_ATTRIBUTE {
        Err(Error::new_spanned(
            path,
            "Only two strategies are available",
        ))
    } else {
        let ident = path
            .get_ident()
            .ok_or(Error::new_spanned(path, "Invalid strategy"))?;
        let strategies = MappingStrategy::str_to_hash_set(ident.to_string().as_ref())
            .map_err(|e| Error::new_spanned(path, e))?;
        Ok(strategies
            .into_iter()
            .map(|strategy| SpannedItem::new(path.clone(), strategy))
            .collect())
    }
}
