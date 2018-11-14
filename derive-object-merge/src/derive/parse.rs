use derive::tree::{ObjectMember, ObjectMerge, MemberAction};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{
    DataStruct, DeriveInput, Field, Fields, Ident, Index, Lit, Member, Meta, NestedMeta,
    PredicateType, TraitBound, TypeParamBound, WherePredicate,
};

impl<'a> ObjectMerge<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataStruct) -> Result<Self, String> {
        let name = &input.ident;

        let members = ObjectMember::parse_all(&data.fields)?;

        let mut generics = input.generics.clone();
        {
            let clause = generics.make_where_clause();
            for member in &members {
                if let Some(predicate) = member.trait_bound() {
                    clause.predicates.push(predicate);
                }
            }
        }

        Ok(ObjectMerge {
            name,
            members,
            generics,
        })
    }
}

impl<'a> ObjectMember<'a> {
    fn parse_all(fields: &'a Fields) -> Result<Vec<Self>, String> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Self::parse(field, i))
            .collect()
    }

    fn parse(field: &'a Field, index: usize) -> Result<Self, String> {
        let member = field
            .ident
            .clone()
            .map(Member::Named)
            .unwrap_or_else(|| Self::make_index(field, index));

        let ty = &field.ty;

        let action = MemberAction::parse(field)?;

        Ok(ObjectMember { member, ty, action })
    }

    fn make_index(field: &Field, index: usize) -> Member {
        syn::Member::Unnamed(Index {
            index: index as u32,
            span: field.span(),
        })
    }

    fn trait_bound(&self) -> Option<WherePredicate> {
        let action_bound = self.action.trait_bound()?;
        let action_bound = TypeParamBound::Trait(action_bound);

        let predicate = PredicateType {
            lifetimes: None,
            bounded_ty: self.ty.clone(),
            colon_token: Token![:](Span::call_site()),
            bounds: Some(action_bound).into_iter().collect(),
        };

        Some(predicate.into())
    }
}

impl MemberAction {
    fn parse(field: &Field) -> Result<Self, String> {
        for attr in &field.attrs {
            if attr.path.segments.len() != 1 {
                continue;
            }

            let attr_id = &attr.path.segments[0].ident;
            let meta = if attr_id == "ignore" {
                return Ok(MemberAction::Ignore);
            } else if attr_id == "merge" {
                return Ok(MemberAction::Merge);
            } else if attr_id == "shallow_merge" {
                return Ok(MemberAction::ShallowMerge);
            } else if attr_id == "combine" {
                return Ok(MemberAction::Combine);
            } else if attr_id == "merge_combine" {
                return Ok(MemberAction::MergeCombine);
            } else if attr_id == "merge_by"
                || attr_id == "combine_by"
                || attr_id == "merge_combine_by"
            {
                match attr.parse_meta() {
                    Ok(meta) => meta,
                    _ => return Err(format!("Invalid syntax for #[{}]", attr_id)),
                }
            } else {
                continue;
            };

            let list = match &meta {
                Meta::List(list) => list,
                _ => return Err(format!("Invalid syntax for #[{}]", attr_id)),
            };

            if list.nested.len() != 1 {
                return Err(format!("Invalid syntax for #[{}]", attr_id));
            }

            let member = match &list.nested[0] {
                NestedMeta::Meta(Meta::List(inner)) => {
                    if inner.ident != "member" || inner.nested.len() != 1 {
                        return Err(format!("Invalid syntax for #[{}]", attr_id));
                    }

                    match &inner.nested[0] {
                        NestedMeta::Meta(Meta::Word(member)) => Member::Named(member.clone()),
                        NestedMeta::Literal(Lit::Str(lit)) => {
                            Member::Named(Ident::new(&lit.value(), lit.span()))
                        }
                        NestedMeta::Literal(Lit::Int(lit)) => Member::Unnamed(Index {
                            index: lit.value() as u32,
                            span: lit.span(),
                        }),
                        _ => return Err(format!("Invalid syntax for #[{}]", attr_id)),
                    }
                }
                NestedMeta::Meta(Meta::NameValue(inner)) => {
                    if inner.ident != "member" {
                        return Err(format!("Invalid syntax for #[{}]", attr_id));
                    }

                    match &inner.lit {
                        Lit::Str(lit) => Member::Named(Ident::new(&lit.value(), lit.span())),
                        Lit::Int(lit) => Member::Unnamed(Index {
                            index: lit.value() as u32,
                            span: lit.span(),
                        }),
                        _ => return Err(format!("Invalid syntax for #[{}]", attr_id)),
                    }
                }
                _ => return Err(format!("Invalid syntax for #[{}]", attr_id)),
            };

            let lambda = quote! { |item| item.#member.clone() };

            if attr_id == "merge_by" {
                return Ok(MemberAction::MergeByKey(lambda));
            } else if attr_id == "combine_by" {
                return Ok(MemberAction::CombineByKey(lambda));
            } else if attr_id == "merge_combine_by" {
                return Ok(MemberAction::MergeCombineByKey(lambda));
            } else {
                unreachable!()
            }
        }

        Ok(MemberAction::Merge)
    }

    fn trait_bound(&self) -> Option<TraitBound> {
        let bound = match self {
            MemberAction::Ignore => return None,
            MemberAction::Merge => quote! { object_merge::Merge },
            MemberAction::ShallowMerge => quote! { object_merge::ShallowMerge },
            MemberAction::Combine => quote! { object_merge::Combine },
            MemberAction::MergeCombine => quote! { object_merge::MergeCombine },
            MemberAction::MergeByKey(_) => quote! { object_merge::MergeByKey },
            MemberAction::CombineByKey(_) => quote! { object_merge::CombineByKey },
            MemberAction::MergeCombineByKey(_) => quote! { object_merge::MergeCombineByKey },
        };
        let bound: ::proc_macro::TokenStream = bound.into();
        let bound = syn::parse_macro_input::parse(bound).unwrap();
        Some(bound)
    }
}
