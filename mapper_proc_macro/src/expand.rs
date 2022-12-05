use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::DeriveInput;
use syn::Result;

use crate::ast::Input;
use crate::ast::Struct;

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    dbg!("{}", &input);
    input.validate()?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
    })
}

fn impl_struct(input: Struct) -> TokenStream {
    let ty = &input.ident;
    let (_impl_generics, _ty_generics, _where_clause) = input.generics.split_for_impl();
    let mut tokens = TokenStream::new();

    let fields = input
        .fields
        .iter()
        .map(|field| {
            let field_original = &field.member;
            quote::quote! {
                #field_original:self.#field_original.clone()
            }
        })
        .collect::<Vec<TokenStream>>();

    if let Some(to) = input.attrs.to {
        let implementations = to
            .destinations
            .iter()
            .map(|d| {
                quote::quote! {
                    impl Mapper<#d> for #ty {
                        fn to(&self)->#d{
                            #d{
                                #(#fields)*,
                            }
                        }
                    }
                }
            })
            .collect::<Vec<TokenStream>>();
        tokens.append_all(&implementations);
    }
    tokens
}
