use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField, ast::Data, util::Ignored};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Ident, Type, TypePath};

use crate::error::GeneratorResult;

#[derive(FromField)]
#[darling(attributes(js), forward_attrs(doc))]
struct ObjectField {
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<Attribute>,

    #[darling(default)]
    array: bool,
    #[darling(default)]
    opt: bool,
    #[darling(default)]
    datetime: bool,
    #[darling(default)]
    sub_types: bool,
    #[darling(default)]
    derivative_types: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(js), forward_attrs(doc))]
struct ObjectArgs {
    ident: Ident,
    data: Data<Ignored, ObjectField>,

    remote: TypePath,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let ObjectArgs {
        ident,
        data,
        remote,
    } = ObjectArgs::from_derive_input(&args)?;

    let s = match data {
        Data::Struct(s) => s,
        _ => return Err(Error::new_spanned(ident, "can only be applied to an struct").into()),
    };

    let mut fields = Vec::new();
    let mut getters = Vec::new();
    let mut from_fields = Vec::new();
    let mut json_fields = Vec::new();

    for field in &s.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let attrs = &field.attrs;

        fields.push(field_ident);
        getters.push(quote! {
            #[napi(getter)]
            #[inline]
            #(#attrs)*
            pub fn #field_ident(&self) -> #field_type {
                self.#field_ident.clone()
            }
        });
        let name = field_ident.to_string().to_case(Case::Camel);
        json_fields.push(quote! {
            (#name.to_string(), <#field_type as crate::utils::ToJSON>::to_json(&self.#field_ident))
        });

        if field.sub_types {
            from_fields.push(quote! {
                #field_ident: crate::quote::types::SubTypes::from(#field_ident).0,
            });
            continue;
        }

        if field.derivative_types {
            from_fields.push(quote! {
                #field_ident: crate::quote::types::DerivativeTypes::from(#field_ident).0,
            });
            continue;
        }

        match (field.array, field.opt, field.datetime) {
            (true, false, false) => {
                from_fields.push(quote! {
                    #field_ident: #field_ident
                        .into_iter()
                        .map(TryInto::try_into)
                        .collect::<::std::result::Result<::std::vec::Vec<_>, _>>()?,
                });
            }
            (false, true, false) => {
                from_fields.push(quote! {
                    #field_ident: match #field_ident {
                        ::std::option::Option::Some(value) => ::std::option::Option::Some(value.try_into()?),
                        ::std::option::Option::None => ::std::option::Option::None,
                    },
                });
            }
            (false, false, false) => {
                from_fields.push(quote! {
                    #field_ident: #field_ident.try_into()?,
                });
            }
            (true, false, true) => {
                from_fields.push(quote! {
                    #field_ident: #field_ident
                        .into_iter()
                        .map(crate::utils::to_datetime)
                        .collect::<::std::result::Result<::std::vec::Vec<_>, _>>()?,
                });
            }
            (false, true, true) => {
                from_fields.push(quote! {
                    #field_ident: #field_ident.map(crate::utils::to_datetime),
                });
            }
            (false, false, true) => {
                from_fields.push(quote! {
                    #field_ident: crate::utils::to_datetime(#field_ident),
                });
            }
            _ => return Err(Error::new_spanned(ident, "invalid attributes").into()),
        }
    }

    let expanded = quote! {
        #[napi_derive::napi]
        impl #ident {
            #[napi]
            pub fn to_string(&self) -> String {
                ::std::format!("{:?}", self)
            }

            #[napi(js_name = "toJSON")]
            pub fn to_json(&self) -> serde_json::Value {
                <Self as crate::utils::ToJSON>::to_json(self)
            }

            #(#getters)*
        }

        impl ::std::convert::TryFrom<#remote> for #ident {
            type Error = ::napi::Error;

            fn try_from(#remote { #(#fields),* }: #remote) -> ::std::result::Result<Self, Self::Error> {
                use ::std::convert::TryInto;
                use ::std::iter::Iterator;

                Ok(Self {
                    #(#from_fields)*
                })
            }
        }

        impl crate::utils::ToJSON for #ident {
            fn to_json(&self) -> serde_json::Value {
                serde_json::Value::Object([#(#json_fields),*].into_iter().collect())
            }
        }
    };

    Ok(expanded)
}
