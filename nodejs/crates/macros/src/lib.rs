mod error;
mod jsenum;
mod jsobject;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(JsObject, attributes(js))]
pub fn derive_pyobject(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match jsobject::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(JsEnum, attributes(js))]
pub fn derive_jsenum(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match jsenum::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}
