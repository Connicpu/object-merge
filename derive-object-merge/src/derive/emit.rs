use crate::derive::tree::{ObjectMember, ObjectMerge};

use proc_macro2::TokenStream;

impl<'a> ObjectMerge<'a> {
    pub fn emit(&self) -> TokenStream {
        let name = self.name;
        let (impgen, tygen, wherec) = self.generics.split_for_impl();
        let merge_members = self.members.iter().map(|member| member.emit_merge());

        quote! {
            impl #impgen object_merge::Merge for #name #tygen #wherec {
                fn merge(&mut self, template: &Self) {
                    #( #merge_members )*
                }
            }
        }
    }
}

impl<'a> ObjectMember<'a> {
    pub fn emit_merge(&self) -> TokenStream {
        use crate::derive::tree::MemberAction::*;

        let field = &self.member;
        let ty = self.ty;

        match &self.action {
            Ignore => quote! {},
            Merge => quote! {
                <#ty as object_merge::Merge>::merge(
                    &mut self.#field,
                    &template.#field
                );
            },
            ShallowMerge => quote! {
                <#ty as object_merge::ShallowMerge>::shallow_merge(
                    &mut self.#field,
                    &template.#field,
                );
            },
            ShallowOverwrite => quote! {
                <#ty as object_merge::ShallowOverwrite>::shallow_overwrite(
                    &mut self.#field,
                    &template.#field,
                );
            },
            Overwrite => quote! {
                <#ty as object_merge::Overwrite>::overwrite(
                    &mut self.#field,
                    &template.#field,
                );
            },
            Combine => quote! {
                <#ty as object_merge::Combine>::combine(
                    &mut self.#field,
                    &template.#field
                );
            },
            MergeCombine => quote! {
                <#ty as object_merge::MergeCombine>::merge_combine(
                    &mut self.#field,
                    &template.#field
                );
            },
            MergeByKey(key_fn) => quote! {
                <#ty as object_merge::MergeByKey>::merge_by_key(
                    &mut self.#field,
                    &template.#field,
                    #key_fn
                )
            },
            CombineByKey(key_fn) => quote! {
                <#ty as object_merge::CombineByKey>::combine_by_key(
                    &mut self.#field,
                    &template.#field,
                    #key_fn
                )
            },
            MergeCombineByKey(key_fn) => quote! {
                <#ty as object_merge::MergeCombineByKey>::merge_combine_by_key(
                    &mut self.#field,
                    &template.#field,
                    #key_fn
                )
            },
        }
    }
}
