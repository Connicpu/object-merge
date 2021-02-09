use crate::{
    Combine, CombineByKey, Merge, MergeByKey, MergeCombine, MergeCombineByKey, ShallowMerge,
    ShallowOverwrite,
};

use std::hash::Hash;

fn merge_with<T>(lhs: &mut Option<T>, rhs: &Option<T>, merge: impl FnOnce(&mut T, &T))
where
    T: Clone,
{
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => merge(lhs, rhs),
        (lhs @ None, rhs) => *lhs = rhs.clone(),
        (_, None) => (),
    }
}

impl<T> Merge for Option<T>
where
    T: Merge + Clone,
{
    fn merge(&mut self, template: &Self) {
        merge_with(self, template, |lhs, rhs| lhs.merge(rhs));
    }
}

impl<T> ShallowMerge for Option<T>
where
    T: Clone,
{
    fn shallow_merge(&mut self, template: &Self) {
        merge_with(self, template, |_, _| ());
    }
}

impl<T> ShallowOverwrite for Option<T>
where
    T: Clone,
{
    fn shallow_overwrite(&mut self, template: &Self) {
        merge_with(self, template, |lhs, rhs| *lhs = rhs.clone());
    }
}

impl<T> Combine for Option<T>
where
    T: Combine + Clone,
{
    fn combine(&mut self, template: &Self) {
        merge_with(self, template, |lhs, rhs| lhs.combine(rhs));
    }
}

impl<T> MergeCombine for Option<T>
where
    T: MergeCombine + Clone,
{
    fn merge_combine(&mut self, template: &Self) {
        merge_with(self, template, |lhs, rhs| lhs.merge_combine(rhs));
    }
}

impl<T> MergeByKey for Option<T>
where
    T: MergeByKey + Clone,
{
    type Elem = T::Elem;

    fn merge_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq,
    {
        merge_with(self, template, |lhs, rhs| lhs.merge_by_key(rhs, get_key));
    }
}

impl<T> MergeCombineByKey for Option<T>
where
    T: MergeCombineByKey + Clone,
{
    type Elem = T::Elem;

    fn merge_combine_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq,
    {
        merge_with(self, template, |lhs, rhs| {
            lhs.merge_combine_by_key(rhs, get_key)
        });
    }
}

impl<T> CombineByKey for Option<T>
where
    T: CombineByKey + Clone,
{
    type Elem = T::Elem;

    fn combine_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq,
    {
        merge_with(self, template, |lhs, rhs| lhs.combine_by_key(rhs, get_key));
    }
}
