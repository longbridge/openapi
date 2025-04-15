use darling::{
    FromDeriveInput, FromVariant,
    ast::{Data, Fields},
    util::Ignored,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Ident, TypePath};

use crate::error::GeneratorResult;

#[derive(FromVariant)]
#[darling(attributes(py), forward_attrs(doc))]
struct EnumItem {
    ident: Ident,
    fields: Fields<Ignored>,

    #[darling(default)]
    remote: Option<Ident>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(py), forward_attrs(doc))]
struct EnumArgs {
    ident: Ident,
    data: Data<EnumItem, Ignored>,

    remote: TypePath,
    #[darling(default)]
    from: Option<bool>,
    #[darling(default)]
    into: Option<bool>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let EnumArgs {
        ident,
        data,
        remote,
        from,
        into,
    } = EnumArgs::from_derive_input(&args)?;
    let from = from.unwrap_or(true);
    let into = into.unwrap_or(true);

    let e = match data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(ident, "can only be applied to an enum").into()),
    };

    let mut from_remote = Vec::new();
    let mut from_local = Vec::new();

    for variant in e {
        if !variant.fields.is_empty() {
            return Err(Error::new_spanned(
                &variant.ident,
                format!("invalid enum variant {}.", variant.ident),
            )
            .into());
        }

        let item_ident = &variant.ident;
        let remote_ident = variant.remote.as_ref().unwrap_or(&variant.ident);

        from_remote.push(quote! {
            #remote::#remote_ident => #ident::#item_ident,
        });
        from_local.push(quote! {
            #ident::#item_ident => #remote::#remote_ident,
        });
    }

    let impl_from = if from {
        Some(quote! {
            impl ::std::convert::From<#remote> for #ident {
                fn from(value: #remote) -> #ident {
                    match value {
                        #(#from_remote)*
                    }
                }
            }
        })
    } else {
        None
    };

    let impl_into = if into {
        Some(quote! {
            impl ::std::convert::From<#ident> for #remote {
                fn from(value: #ident) -> #remote {
                    match value {
                        #(#from_local)*
                    }
                }
            }
        })
    } else {
        None
    };

    let expanded = quote! {
        #impl_from
        #impl_into
    };

    Ok(expanded)
}
