use quote::{TokenStreamExt, ToTokens};

use crate::{ast::mapping_tree::MappingTree, attr::mapping_strategy::MappingStrategy};


impl MappingTree{
    pub fn expand(&self, tokens: &mut  proc_macro2::TokenStream)  {
        let dest = &self.destination;
        let ty = &self.ident;
        let fields = self.mapping_fields.iter().map(|f| {
            let destination = f.get_dest_field();
            let value = f.get_src_field();
            quote::quote! {
                #destination:#value
            }
        });
        let implementation = match self.strategy {
            MappingStrategy::Into => quote::quote! {
                impl Into<#dest> for #ty{
                    fn into(self) -> #dest{
                        #dest{
                            #(#fields),*
                        }
                    }
                }
            },
            MappingStrategy::Mapper => quote::quote! {
                impl Mapper<#dest> for #ty {
                    fn to(&self)->#dest{
                        #dest{
                            #(#fields),*
                        }
                    }
                }
            },
        };
        implementation.to_tokens(tokens);
    }
} 
