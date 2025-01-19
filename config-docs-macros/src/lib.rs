use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

struct Documentation {
    name: syn::Ident,
    fields: Vec<Field>,
}

enum Field {
    Simple(syn::Ident, String),
    Nested(syn::Ident, syn::Type),
}

impl From<DeriveInput> for Documentation {
    fn from(input: DeriveInput) -> Self {
        if let DeriveInput {
            ident,
            data:
                syn::Data::Struct(syn::DataStruct {
                    fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }),
                    ..
                }),
            ..
        } = input
        {
            let fields = fields.into_iter().map(Into::into).collect();
            Self {
                name: ident,
                fields,
            }
        } else {
            panic!("only works for structs");
        }
    }
}

impl ToTokens for Documentation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Documentation { name, fields } = self;
        let out = quote! {
            impl ::config_docs::ConfigDocs for #name {
                const CONFIG_DOCS: ::config_docs::ConfigDocumentation = ::config_docs::ConfigDocumentation(&[#(#fields ,)*]);
            }
        };
        tokens.extend(out);
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let out = match self {
            Field::Simple(ident, description) => {
                let ident = ident.to_string();
                quote! {
                    ::config_docs::ConfigDocumentationPart::Line(#ident, #description)
                }
            },
            Field::Nested(ident, ty) => {
                let ident = ident.to_string();

                quote! {
                    ::config_docs::ConfigDocumentationPart::SubPart(#ident, &<#ty as ::config_docs::ConfigDocs>::CONFIG_DOCS)
                }
            },
        };
        tokens.extend(out);
    }
}

impl From<syn::Field> for Field {
    fn from(value: syn::Field) -> Self {
        let syn::Field {
            attrs, ident, ty, ..
        } = value;

        let ident = ident.expect("Should be a named field");
        let is_nested = attrs.iter().any(|attr| attr.path().is_ident("nested"));
        if is_nested {
            Field::Nested(ident, ty)
        } else {
            let lines: Vec<_> = extract_doc_comment(&attrs);
            let description = lines.into_iter().next().unwrap_or_default();
            Field::Simple(ident, description)
        }
    }
}

fn extract_doc_comment(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let syn::Meta::NameValue(syn::MetaNameValue {
                value:
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }),
                ..
            }) = &attr.meta
            {
                Some(s.value())
            } else {
                None
            }
        })
        .map(|mut s| {
            if s.starts_with(" ") {
                s.split_off(1)
            } else {
                s
            }
        })
        .collect()
}

/// Derives `ConfigDocs` using the provided doc-comments
#[proc_macro_derive(ConfigDocs, attributes(nested))]
pub fn derive_config_docs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let docs: Documentation = parse_macro_input!(input as DeriveInput).into();

    quote::quote! { #docs }.into()
}
