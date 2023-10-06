extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod pack;
mod size;
mod unpack;

#[proc_macro_derive(ByteSize)]
pub fn bytesize_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    size::impl_bytesize(&ast)
}

#[proc_macro_derive(BytePack)]
pub fn bytepack_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    pack::impl_bytepack(&ast)
}

#[proc_macro_derive(ByteUnpack)]
pub fn byteunpack_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    unpack::impl_byteunpack(&ast)
}
