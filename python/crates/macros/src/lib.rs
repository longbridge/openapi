mod error;
mod pyenum;
mod pyobject;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(PyObject, attributes(py))]
pub fn derive_pyobject(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match pyobject::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(PyEnum, attributes(py))]
pub fn derive_pyenum(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match pyenum::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}
