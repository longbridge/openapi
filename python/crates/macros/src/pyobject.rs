use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Ident, Type, TypePath};

use crate::error::GeneratorResult;

#[derive(FromField)]
#[darling(attributes(py), forward_attrs(doc))]
struct ObjectField {
    ident: Option<Ident>,
    ty: Type,

    #[darling(default)]
    array: bool,
    #[darling(default)]
    opt: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(py), forward_attrs(doc))]
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

        getters.push(quote! {
            #[getter]
            #[inline]
            fn #field_ident(&self) -> #field_type {
                self.#field_ident.clone()
            }
        });

        if field.array {
            from_fields.push(quote! {
                #field_ident: value.#field_ident
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<::std::vec::Vec<_>, _>>()?,
            });
        } else if field.opt {
            from_fields.push(quote! {
                #field_ident: match value.#field_ident {
                    ::std::option::Option::Some(value) => ::std::option::Option::Some(value.try_into()?),
                    ::std::option::Option::None => ::std::option::Option::None,
                },
            });
        } else {
            from_fields.push(quote! {
                #field_ident: value.#field_ident.try_into()?,
            });
        }
    }

    let expanded = quote! {
        #[::pyo3::pymethods]
        impl #ident {
            fn __repr__(&self) -> String {
                ::std::format!("{:?}", self)
            }

            fn __str__(&self) -> String {
                ::std::format!("{:?}", self)
            }

            #(#getters)*
        }

        impl ::std::convert::TryFrom<#remote> for #ident {
            type Error = ::pyo3::PyErr;

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
