#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod derive;

#[proc_macro_derive(
    Merge,
    attributes(
        ignore,
        shallow_merge,
        merge,
        combine,
        merge_combine,
        merge_by,
        combine_by,
        merge_combine_by
    )
)]
pub fn derive_merge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive::expand_derive(&input)
        .unwrap_or_else(compile_error)
        .into()
}

fn compile_error(message: String) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#message);
    }
}
