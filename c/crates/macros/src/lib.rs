mod cenum;
mod error;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(CEnum, attributes(c))]
pub fn derive_cenum(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match cenum::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}
