use syn::{Generics, Ident, Type};
use proc_macro2::TokenStream;

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
    Combine,
    MergeCombine,
    MergeByKey(TokenStream),
    CombineByKey(TokenStream),
    MergeCombineByKey(TokenStream),
}
