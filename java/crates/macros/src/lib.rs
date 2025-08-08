use darling::FromMeta;
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Error, Expr, ExprArray, ExprLit, ExprPath, Ident, Lit, Path, parse::Parser,
    punctuated::Punctuated, token::Comma,
};

#[derive(FromMeta, Debug, Default)]
struct FieldArgs {
    #[darling(default)]
    objarray: bool,
    #[darling(default)]
    priarray: bool,
    #[darling(default)]
    derivative_types: bool,
}

#[derive(FromMeta, Debug, Default)]
struct EnumItemArgs {
    #[darling(default)]
    remote: Option<Path>,
}

#[proc_macro]
pub fn impl_java_class(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Comma>::parse_separated_nonempty;
    let mut exprs = match parser.parse(input) {
        Ok(exprs) => exprs.into_iter(),
        Err(err) => return err.to_compile_error().into(),
    };

    let classname = {
        let expr = exprs.next();
        match &expr {
            Some(Expr::Lit(ExprLit {
                lit: Lit::Str(str), ..
            })) => str.value(),
            _ => {
                return Error::new_spanned(&expr, "missing class name")
                    .to_compile_error()
                    .into();
            }
        }
    };

    let type_path = {
        let expr = exprs.next();
        match &expr {
            Some(Expr::Path(ExprPath { path, .. })) => path.clone(),
            _ => {
                return Error::new_spanned(&expr, "missing remote type")
                    .to_compile_error()
                    .into();
            }
        }
    };

    let fields = {
        let expr = exprs.next();
        let mut fields = Vec::new();

        match &expr {
            Some(Expr::Array(ExprArray { elems, .. })) => {
                for elem in elems {
                    match elem {
                        Expr::Path(ExprPath { path, attrs, .. }) => {
                            let args = attrs
                                .iter()
                                .find(|attr| attr.path().is_ident("java"))
                                .and_then(|attr| FieldArgs::from_meta(&attr.meta).ok())
                                .unwrap_or_default();
                            fields.push((path.clone(), args));
                        }
                        _ => {
                            return Error::new_spanned(elem, "invalid field")
                                .to_compile_error()
                                .into();
                        }
                    }
                }
            }
            _ => {
                return Error::new_spanned(&expr, "missing field list")
                    .to_compile_error()
                    .into();
            }
        }

        fields
    };

    let signature = format!("L{classname};");
    let mut set_fields = Vec::new();

    for (field, args) in &fields {
        let ident = match field.get_ident() {
            Some(ident) => ident,
            None => {
                return Error::new_spanned(field, "invalid field")
                    .to_compile_error()
                    .into();
            }
        };
        let java_field = ident.to_string().to_camel_case();

        if args.derivative_types {
            set_fields.push(quote! {
                crate::types::set_field(env, &obj, #java_field, crate::types::enum_types::DerivativeTypes::from(#ident))?;
            });
        } else if args.objarray {
            set_fields.push(quote! {
                crate::types::set_field(env, &obj, #java_field, crate::types::ObjectArray(#ident))?;
            });
        } else if args.priarray {
            set_fields.push(quote! {
                crate::types::set_field(env, &obj, #java_field, crate::types::PrimaryArray(#ident))?;
            });
        } else {
            set_fields.push(quote! {
                crate::types::set_field(env, &obj, #java_field, #ident)?;
            });
        }
    }

    let field_names = fields.iter().map(|(field, _)| field).collect::<Vec<_>>();
    let class_ref_name = Ident::new(&classname.replace('/', "_"), Span::call_site());
    let def_class_ref = quote! {
        #[allow(non_upper_case_globals)]
        static #class_ref_name: std::sync::OnceLock<jni::objects::GlobalRef> = std::sync::OnceLock::new();
    };

    let expanded = quote! {
        #def_class_ref

        impl crate::types::ClassLoader for #type_path {
            fn init(env: &mut jni::JNIEnv) {
                let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(#classname, env).expect(#classname);
                let _ = #class_ref_name.set(env.new_global_ref(&*cls).unwrap());
            }

            fn class_ref() -> jni::objects::GlobalRef {
                #class_ref_name.get().cloned().unwrap()
            }
        }

        impl crate::types::JSignature for #type_path {
            #[inline]
            fn signature() -> ::std::borrow::Cow<'static, str> {
                #signature.into()
            }
        }

        impl crate::types::IntoJValue for #type_path {
            fn into_jvalue<'a>(self, env: &mut jni::JNIEnv<'a>) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
                let #type_path { #(#field_names),* } = self;
                let cls = <Self as crate::types::ClassLoader>::class_ref();
                let obj = env.new_object(cls.borrow(), "()V", &[])?;
                #(#set_fields)*
                Ok(obj.into())
            }
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn impl_java_enum(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Comma>::parse_separated_nonempty;
    let mut exprs = match parser.parse(input) {
        Ok(exprs) => exprs.into_iter(),
        Err(err) => return err.to_compile_error().into(),
    };

    let classname = {
        let expr = exprs.next();
        match &expr {
            Some(Expr::Lit(ExprLit {
                lit: Lit::Str(str), ..
            })) => str.value(),
            _ => {
                return Error::new_spanned(&expr, "missing class name")
                    .to_compile_error()
                    .into();
            }
        }
    };

    let type_path = {
        let expr = exprs.next();
        match &expr {
            Some(Expr::Path(ExprPath { path, .. })) => path.clone(),
            _ => {
                return Error::new_spanned(&expr, "missing remote type")
                    .to_compile_error()
                    .into();
            }
        }
    };

    let items = {
        let expr = exprs.next();
        let mut items = Vec::new();

        match &expr {
            Some(Expr::Array(ExprArray { elems, .. })) => {
                for elem in elems {
                    match elem {
                        Expr::Path(ExprPath { path, attrs, .. }) => {
                            let args = attrs
                                .iter()
                                .find(|attr| attr.path().is_ident("java"))
                                .and_then(|attr| EnumItemArgs::from_meta(&attr.meta).ok())
                                .unwrap_or_default();
                            items.push((path.clone(), args));
                        }
                        _ => {
                            return Error::new_spanned(elem, "invalid enum item")
                                .to_compile_error()
                                .into();
                        }
                    }
                }
            }
            _ => {
                return Error::new_spanned(&expr, "missing enum items")
                    .to_compile_error()
                    .into();
            }
        }

        items
    };

    let mut from_jsvalue = Vec::new();
    let mut into_jsvalue = Vec::new();

    for (item, args) in items {
        let java_path = &item;
        let remote_path = args.remote.as_ref().unwrap_or(&item);

        from_jsvalue.push(quote! {
            let r = env.get_static_field(&cls, stringify!(#java_path), concat!("L", #classname, ";"))?.l()?;
            if env.is_same_object(&value, r)? {
                return Ok(#remote_path);
            }
        });

        into_jsvalue.push(quote! {
            #remote_path => env.get_static_field(&cls, stringify!(#java_path), concat!("L", #classname, ";")),
        });
    }

    let class_ref_name = Ident::new(&classname.replace('/', "_"), Span::call_site());
    let def_class_ref = quote! {
        #[allow(non_upper_case_globals)]
        static #class_ref_name: std::sync::OnceLock<jni::objects::GlobalRef> = std::sync::OnceLock::new();
    };

    let expanded = quote! {
        #def_class_ref

        impl crate::types::ClassLoader for #type_path {
            fn init(env: &mut jni::JNIEnv) {
                let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(#classname, env).expect(#classname);
                let _ = #class_ref_name.set(env.new_global_ref(&*cls).unwrap());
            }

            fn class_ref() -> jni::objects::GlobalRef {
                #class_ref_name.get().cloned().unwrap()
            }
        }

        impl crate::types::JSignature for #type_path {
            #[inline]
            fn signature() -> std::borrow::Cow<'static, str> {
                concat!("L", #classname, ";").into()
            }
        }

        impl crate::types::FromJValue for #type_path {
            fn from_jvalue(
                env: &mut jni::JNIEnv,
                value: jni::objects::JValueOwned,
            ) -> jni::errors::Result<Self> {
                use #type_path::*;
                let cls = <Self as crate::types::ClassLoader>::class_ref();
                let value = value.l()?;
                #(#from_jsvalue)*
                panic!("invalid enum value")
            }
        }

        impl crate::types::IntoJValue for #type_path {
            fn into_jvalue<'a>(
                self,
                env: &mut jni::JNIEnv<'a>,
            ) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
                use #type_path::*;

                let cls = <Self as crate::types::ClassLoader>::class_ref();
                match self {
                    #(#into_jsvalue)*
                }
            }
        }
    };

    expanded.into()
}
