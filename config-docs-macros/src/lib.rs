extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Field, Lit, Meta, MetaNameValue};

/// Derives `ConfigDocs` using the provided doc-comments
#[proc_macro_derive(ConfigDocs)]
pub fn derive_config_docs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = if let Data::Struct(data) = input.data {
        let field_docs = data.fields.iter().map(|field| {
            let field_name = field
                .ident
                .as_ref()
                .map_or_else(|| String::from(""), |ident| ident.to_string());

            let doc_comment = get_doc_string(field);
            let ty = &field.ty;

            Some(quote! {
                docs.push((#field_name, #doc_comment));
                docs.extend(<#ty as ConfigDocs>::config_docs());
            })
        });

        quote! {
            impl ConfigDocs for #struct_name {
                fn config_docs() -> &'static [(&'static str, &'static str)] {
                    let mut docs = Vec::new();
                    #(#field_docs)*
                    Box::leak(docs.into_boxed_slice())
                }
            }
        }
    } else {
        quote! {
            compile_error!("ConfigDocs can only be derived for structs");
        }
    };

    expanded.into()
}

fn parse_doc_string(field: &Field) -> Vec<String> {
    let doc_strs = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ..
            }) = &attr.meta
            {
                return Some(s.value());
            }
            None
        })
        .map(|mut s| {
            if s.starts_with(" ") {
                s.split_off(1)
            } else {
                s
            }
        })
        .map(|s| s.to_string())
        .skip_while(|s| s.is_empty())
        .collect();

    doc_strs
}

fn get_doc_string(field: &Field) -> String {
    parse_doc_string(field)
        .into_iter()
        .next()
        .unwrap_or_default()
}
