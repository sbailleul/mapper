use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{Error, Path, Attribute};
use syn::{Expr, Token};
use thiserror::Error;

#[derive(Debug)]
pub struct Attrs<'a> {
    pub to: Vec<To<'a>>,
}

#[derive(Clone, Debug)]
pub struct To<'a> {
    pub original: &'a Attribute,
    pub params: Params
}

#[derive(Clone, Debug)]
pub struct Params {
    pub ty: Path,
    pub field: Option<Path>,
    pub with: Option<Path>,
}

#[derive(Error, Debug)]
pub enum ToCreationError {
    #[error("to attribute should be used with at least field or with")]
    MissingConfigField,
    #[error("to attribute destination type is mandatory")]
    MissingDestination,
}

impl Params {
    pub fn new(
        ty: Option<Path>,
        field: Option<Path>,
        with: Option<Path>,
    ) -> Result<Self, ToCreationError> {
        if field.is_none() && with.is_none() {
            Err(ToCreationError::MissingConfigField)
        } else if ty.is_none() {
            Err(ToCreationError::MissingDestination)
        } else {
            Ok(Self {
                ty: ty.unwrap(),
                field,
                with,
            })
        }
    }
}

impl Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut field: Option<Path> = None;
        let mut with: Option<Path> = None;
        let mut ty: Option<Path> = None;
        let args = Punctuated::<Expr, Token![,]>::parse_separated_nonempty(input)
            .expect("Attribute shouldn't be empty");
        for arg in args {
            match arg {
                Expr::Assign(assign) => {
                    if ty.is_none() {
                        return Err(Error::new_spanned(
                            assign,
                            "Destination type should be specified at first position",
                        ));
                    }
                    parse_config(assign, &mut field, &mut with);
                }
                Expr::Path(path) => {
                    ty = Some(path.path);
                }
                _ => (),
            }
        }
        Params::new(ty, field, with).map_err(|err| syn::Error::new(input.span(), err))
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

pub fn get(input: &syn::Field) -> syn::Result<Attrs> {
    let mut to_attributes = vec![];
    for attr in &input.attrs {
        if attr.path.is_ident("to") {
            to_attributes.push(To{params: attr.parse_args_with(Params::parse)?, original: attr });
        }
    }
    Ok(Attrs { to: to_attributes })
}
