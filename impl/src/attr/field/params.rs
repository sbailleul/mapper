use syn::{
    parse::Parse, punctuated::Punctuated, Error, Expr, ExprPath, Path, Token, Type, TypePath,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParamsError {
    #[error("to attribute should be used with at least field or with")]
    MissingConfigField,
    #[error("excluded attribute couldn't have other configurations fields")]
    ExcludedField,
}
#[derive(Clone, Debug)]
pub struct Params {
    pub ty: TypePath,
    pub field: Option<Path>,
    pub with: Option<Path>,
    pub exclude: bool,
}

impl Params {
    pub fn new(
        ty: TypePath,
        field: Option<Path>,
        with: Option<Path>,
        exclude: bool,
    ) -> Result<Self, ParamsError> {
        if field.is_none() && with.is_none() && !exclude {
            Err(ParamsError::MissingConfigField)
        } else if exclude && (field.is_some() || with.is_some()) {
            Err(ParamsError::ExcludedField)
        } else {
            Ok(Self {
                ty,
                field,
                with,
                exclude,
            })
        }
    }
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut field: Option<Path> = None;
        let mut with: Option<Path> = None;
        let mut exclude: Option<bool> = None;

        if let Ok(Type::Path(ty)) = input.parse::<Type>() {
            if !input.is_empty() {
                input
                    .parse::<Token![,]>()
                    .expect("If there is arguments destination type should be followed by a comma");
            }
            let args = Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)
                .expect("Attribute shouldn't be empty");
            for arg in args {
                match arg {
                    Expr::Assign(assign) => {
                        parse_config(assign, &mut field, &mut with);
                    }
                    Expr::Path(path) => {
                        parse_flag(path, &mut exclude)?;
                    }
                    _ => (),
                }
            }
            Params::new(ty, field, with, exclude.unwrap_or_default()).map_err(|err| syn::Error::new(input.span(), err))
        } else {
            Err(Error::new(
                input.span(),
                "Destination type should be specified at first position",
            ))
        }
    }
}

fn parse_config(assign: syn::ExprAssign, field: &mut Option<Path>, with: &mut Option<Path>) {
    if let Expr::Path(config) = *assign.left {
        if config.path.is_ident("field") {
            if let Expr::Path(dst_field) = *assign.right {
                *field = Some(dst_field.path);
            }
        } else if config.path.is_ident("with") {
            if let Expr::Path(with_fn) = *assign.right {
                *with = Some(with_fn.path);
            }
        }
    }
}

fn parse_flag(path: ExprPath, exclude: &mut Option<bool>) -> syn::Result<()> {
    if path.path.is_ident("exclude") {
        if exclude.is_none() {
            *exclude = Some(true);
        } else {
            return Err(Error::new_spanned(
                path,
                "Cannot specify multiple time exclude flag.",
            ));
        }
    }
    Ok(())
}
