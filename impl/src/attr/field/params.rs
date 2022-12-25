use std::{collections::HashSet, hash::Hash};

use proc_macro2::Ident;
use syn::{
    custom_keyword, parse::Parse, punctuated::Punctuated, token::Comma, Error, Expr, ExprPath,
    Path, Token, Type, TypePath,
};
use thiserror::Error;

use crate::attr::{
    mapping_strategy::{parse_strategy, MappingStrategy},
    spanned_item::SpannedItem,
};

#[derive(Error, Debug)]
pub enum ParamsError {
    #[error("Excluded field attribute couldn't have other configurations fields")]
    ExcludedField,
}
#[derive(Clone, Debug)]
pub struct Params {
    pub destination: Option<TypePath>,
    pub field: Option<Path>,
    pub with: HashSet<SpannedItem<Path, MappingStrategy>>,
    pub exclude: SpannedItem<Path, bool>,
    pub strategies: HashSet<SpannedItem<Path, MappingStrategy>>,
}

impl Params {
    pub fn new(
        destination: Option<TypePath>,
        field: Option<Path>,
        with: HashSet<SpannedItem<Path, MappingStrategy>>,
        exclude: SpannedItem<Path, bool>,
        strategies: HashSet<SpannedItem<Path, MappingStrategy>>,
    ) -> Result<Self, ParamsError> {
        if exclude.1 && (field.is_some() || !with.is_empty() || !strategies.is_empty()) {
            Err(ParamsError::ExcludedField)
        } else {
            Ok(Self {
                destination,
                field,
                with,
                exclude,
                strategies,
            })
        }
    }
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        custom_keyword!(exclude);
        if input.peek(exclude) {
            let exclude_path = input.parse::<Path>().ok();
            if input.is_empty() {
                return Params::new(
                    None,
                    None,
                    HashSet::with_capacity(0),
                    SpannedItem(exclude_path, true),
                    HashSet::with_capacity(0),
                ).map_err(|err| syn::Error::new(input.span(), err));
            } else {
                return Err(Error::new(input.span(), "Cannot exclude field for all destinations if exclude arg isn't the only argument of the attribute"));
            }
        }
        let mut field: Option<Path> = None;
        let mut with = HashSet::new();
        let mut exclude_config = None;
        let mut strategies = HashSet::new();

        if let Ok(Type::Path(ty)) = &input.parse::<Type>() {
            if input.is_empty() {
                Err(Error::new_spanned(
                    ty,
                    "To field attribute should contains configuration",
                ))
            } else {
                input.parse::<Token![,]>().map_err(|_| {
                    Error::new_spanned(
                        ty,
                        "To field attribute destination should be followed by comma",
                    )
                })
            }?;
            let args =
                Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input).map_err(|_| {
                    Error::new_spanned(ty, "To field attribute configuration couldn't be parsed")
                })?;
            for arg in args {
                match arg {
                    Expr::Assign(assign) => {
                        parse_config(assign, &mut field, &mut with, &mut strategies)?;
                    }
                    Expr::Path(path) => {
                        parse_flag(path, &mut exclude_config)?;
                    }
                    _ => (),
                }
            }
            Params::new(
                Some(ty.clone()),
                field,
                with,
                exclude_config.unwrap_or_default(),
                strategies,
            )
            .map_err(|err| syn::Error::new(input.span(), err))
        } else {
            Err(Error::new(
                input.span(),
                "To field attribute destination type should be specified at first position",
            ))
        }
    }
}

fn parse_config(
    assign: syn::ExprAssign,
    field: &mut Option<Path>,
    with: &mut HashSet<SpannedItem<Path, MappingStrategy>>,
    strategies: &mut HashSet<SpannedItem<Path, MappingStrategy>>,
) -> syn::Result<()> {
    match *assign.left {
        Expr::Path(config) => {
            if config.path.is_ident("field") {
                if let Expr::Path(dst_field) = *assign.right {
                    *field = Some(dst_field.path);
                }
            } else if config.path.is_ident("with") {
                parse_with_value(&assign.right, with, None)?;
            } else if config.path.is_ident("strategy") {
                if let Expr::Path(strategy_expr) = *assign.right {
                    let strategy = parse_strategy(&strategy_expr.path, strategies)?;
                    strategies.insert(strategy);
                }
            }
        }
        Expr::Call(config) => {
            if let Expr::Path(func) = *config.func {
                parse_with_strategy(func, &config.args, &assign.right, with)?;
            }
        }
        _ => (),
    }
    Ok(())
}

fn parse_with_strategy(
    func: ExprPath,
    args: &Punctuated<Expr, Comma>,
    value: &syn::Expr,
    with: &mut HashSet<SpannedItem<Path, MappingStrategy>>,
) -> syn::Result<()> {
    if func.path.is_ident("with") {
        if args.len() != 1 {
            Err(Error::new_spanned(
                args,
                "Cannot specify more than on strategy (into or mapper) by with config",
            ))
        } else if let Expr::Path(strategy) = &args[0] {
            let strategy =
                MappingStrategy::try_from(strategy.path.get_ident().unwrap().to_string().as_ref())
                    .map_err(|e| Error::new_spanned(strategy, e))?;
            parse_with_value(value, with, Some(strategy))
        } else {
            Err(Error::new_spanned(
                &args[0],
                "With strategy should be an expression path",
            ))
        }
    } else {
        Ok(())
    }
}

fn parse_with_value(
    value: &syn::Expr,
    with: &mut HashSet<SpannedItem<Path, MappingStrategy>>,
    strategy: Option<MappingStrategy>,
) -> syn::Result<()> {
    if let Expr::Path(with_fn) = value {
        let new_with = SpannedItem::new(with_fn.path.clone(), strategy.unwrap_or_default());
        insert_with(with, new_with, value)
    } else {
        Err(Error::new_spanned(
            value,
            "With value should be a function path",
        ))
    }
}

fn insert_with(
    with: &mut HashSet<SpannedItem<Path, MappingStrategy>>,
    new_with: SpannedItem<Path, MappingStrategy>,
    with_fn: &Expr,
) -> syn::Result<()> {
    if with.contains(&new_with) {
        Err(Error::new_spanned(
            with_fn,
            format!(
                "Cannot add multiple with from same strategy ({})",
                new_with.1
            ),
        ))
    } else {
        with.insert(new_with);
        Ok(())
    }
}

fn parse_flag(
    expr_path: ExprPath,
    exclude: &mut Option<SpannedItem<Path, bool>>,
) -> syn::Result<()> {
    if expr_path.path.is_ident("exclude") {
        if exclude.is_none() {
            *exclude = Some(SpannedItem::new(expr_path.path, true));
        } else {
            return Err(Error::new_spanned(
                expr_path,
                "Cannot specify multiple time exclude flag",
            ));
        }
    }
    Ok(())
}
