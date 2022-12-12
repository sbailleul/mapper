use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::DeriveInput;
use syn::Result;

use crate::ast::Input;
use crate::ast::Struct;

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    input.validate()?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
    })
}

fn impl_struct(input: Struct) -> TokenStream {
    let ty = &input.ident;
    let (_impl_generics, _ty_generics, _where_clause) = input.generics.split_for_impl();
    let mut tokens = TokenStream::new();

    for dest in input.attrs.to.destinations {
        let fields = input.fields
            .iter()
            .map(|field| {
                let destination_field = &field.get_destination_field_by_path(&dest);
                let value = field.get_source_value_by_path(&dest);
                quote::quote! {
                    #destination_field:#value
                }
            })
            .collect::<Vec<TokenStream>>();

        let implementation = quote::quote! {
            impl Mapper<#dest> for #ty {
                fn to(&self)->#dest{
                    #dest{
                        #(#fields)*,
                    }
                }
            }
        };
        tokens.append_all(implementation);
    }

    tokens
}
