use std::{collections::HashSet, rc::Rc};


use syn::{
    parse::{Parse},
    punctuated::Punctuated,
    Error, Expr, Result, Token, Type, TypePath,
};

use crate::{
    attr::mapping_strategy::{parse_strategy, MappingStrategy, MAX_STRATEGIES_BY_ATTRIBUTE},
    common::punctuated_extensions::PunctuatedExtensions,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Params {
    pub destinations: HashSet<TypePath>,
    pub strategies: HashSet<MappingStrategy>,
}

impl  Params {
    fn new(destinations: HashSet<TypePath>,  mut strategies: HashSet<MappingStrategy>) -> Self {
        if strategies.is_empty() {
            strategies.insert(MappingStrategy::default());
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
        let mut strategies: HashSet<MappingStrategy> =
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

fn parse_config(assign: syn::ExprAssign, strategies: &mut HashSet<MappingStrategy>) -> Result<()> {
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
