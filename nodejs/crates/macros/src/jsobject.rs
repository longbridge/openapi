use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
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
        _ => return Err(Error::new_spanned(ident, "Can only be applied to an struct.").into()),
    };

    let mut getters = Vec::new();
    let mut from_fields = Vec::new();

    for field in &s.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let attrs = &field.attrs;

        getters.push(quote! {
            #[napi(getter)]
            #[inline]
            #(#attrs)*
            pub fn #field_ident(&self) -> #field_type {
                self.#field_ident.clone()
            }
        });

        match (field.array, field.opt, field.datetime) {
            (true, false, false) => {
                from_fields.push(quote! {
                    #field_ident: value.#field_ident
                        .into_iter()
                        .map(TryInto::try_into)
                        .collect::<::std::result::Result<::std::vec::Vec<_>, _>>()?,
                });
            }
            (false, true, false) => {
                from_fields.push(quote! {
                    #field_ident: match value.#field_ident {
                        ::std::option::Option::Some(value) => ::std::option::Option::Some(value.try_into()?),
                        ::std::option::Option::None => ::std::option::Option::None,
                    },
                });
            }
            (false, false, false) => {
                from_fields.push(quote! {
                    #field_ident: value.#field_ident.try_into()?,
                });
            }
            (true, false, true) => {
                from_fields.push(quote! {
                    #field_ident: value.#field_ident
                        .into_iter()
                        .map(crate::utils::to_datetime)
                        .collect::<::std::result::Result<::std::vec::Vec<_>, _>>()?,
                });
            }
            (false, true, true) => {
                from_fields.push(quote! {
                    #field_ident: value.#field_ident.map(crate::utils::to_datetime),
                });
            }
            (false, false, true) => {
                from_fields.push(quote! {
                    #field_ident: crate::utils::to_datetime(value.#field_ident),
                });
            }
            _ => return Err(Error::new_spanned(ident, "Invalid attribute.").into()),
        }
    }

    let expanded = quote! {
        #[napi_derive::napi]
        impl #ident {
            #(#getters)*
        }

        impl ::std::convert::TryFrom<#remote> for #ident {
            type Error = ::napi::Error;

            fn try_from(value: #remote) -> ::std::result::Result<Self, Self::Error> {
                use ::std::convert::TryInto;
                use ::std::iter::Iterator;

                Ok(Self {
                    #(#from_fields)*
                })
            }
        }
    };

    Ok(expanded)
}
