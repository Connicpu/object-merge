use proc_macro2::TokenStream;
use syn::{Generics, Ident, Type};

pub struct ObjectMerge<'a> {
    pub name: &'a Ident,
    pub members: Vec<ObjectMember<'a>>,
    pub generics: Generics,
}

pub struct ObjectMember<'a> {
    pub member: syn::Member,
    pub ty: &'a Type,
    pub action: MemberAction,
}

pub enum MemberAction {
    Ignore,
    Merge,
    ShallowMerge,
    ShallowOverwrite,
    Overwrite,
    Combine,
    MergeCombine,
    MergeByKey(TokenStream),
    CombineByKey(TokenStream),
    MergeCombineByKey(TokenStream),
}
