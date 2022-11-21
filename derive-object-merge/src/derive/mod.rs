use crate::derive::tree::ObjectMerge;

use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

pub mod emit;
pub mod parse;
pub mod tree;

pub fn expand_derive(input: &DeriveInput) -> Result<TokenStream, String> {
    let data = match &input.data {
        Data::Struct(input) => input,
        _ => return Err("ObjectMerge may only be derived for structs".into()),
    };

    let tree = ObjectMerge::parse(input, data)?;
    Ok(tree.emit())
}
