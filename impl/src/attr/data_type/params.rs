use std::{collections::HashSet};


use syn::{
    parse::{Parse},
    punctuated::Punctuated, Expr, Result, Token, Type, TypePath, Path,
};

use crate::{
    attr::{mapping_strategy::{parse_strategy, MappingStrategy, MAX_STRATEGIES_BY_ATTRIBUTE}, spanned_item::SpannedItem},
    common::punctuated_extensions::PunctuatedExtensions,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Params {
    pub destinations: HashSet<TypePath>,
    pub strategies: HashSet<SpannedItem<Path, MappingStrategy>>,
}

impl  Params {
    fn new(destinations: HashSet<TypePath>,  mut strategies: HashSet<SpannedItem<Path, MappingStrategy>>) -> Self {
        if strategies.is_empty() {
            strategies.insert(SpannedItem(None, MappingStrategy::default()));
        }
        Params {
            destinations,
            strategies,
        }
    }
}

impl  Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut destinations = HashSet::new();
        let mut strategies =
            HashSet::with_capacity(MAX_STRATEGIES_BY_ATTRIBUTE);

        let args = Punctuated::<Type, Token![,]>::parse_separated_nonempty_until(input, |p| {
            p.peek2(Token![=])
        })
        .expect("Invalid destination types");

        for arg in args {
            match arg {
                Type::Path(ty) => {
                    destinations.insert(ty);
                }
                _ => (),
            }
        }
        if !input.is_empty() {
            let args = Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)
                .expect("Invalid configuration");
            for arg in args {
                match arg {
                    Expr::Assign(assign) => {
                        parse_config(assign, &mut strategies)?;
                    }
                    _ => (),
                }
            }
        }

        Ok(Params::new(destinations, strategies))
    }
}

fn parse_config(assign: syn::ExprAssign, strategies: &mut HashSet<SpannedItem<Path, MappingStrategy>>) -> Result<()> {
    if let Expr::Path(config) = *assign.left {
        if config.path.is_ident("strategy") {
            if let Expr::Path(strategy_expr) = *assign.right {
                let strategy =  parse_strategy(&strategy_expr.path, strategies)?;
                strategies.insert(strategy);
            }
        }
    }
    Ok(())
}
