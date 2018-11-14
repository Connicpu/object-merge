use std::hash::Hash;

mod imp;

/// Combine a type with a template of what values to use when they're unspecified
/// in the `self` instance. On containers like maps, this will recursively call
/// merge for elements that have the same key.
pub trait Merge {
    fn merge(&mut self, template: &Self);
}

/// Used for types like `Option` when no recursive merging should be performed.
pub trait ShallowMerge {
    fn shallow_merge(&mut self, template: &Self);
}

/// Combine two containers by combining all of their elements. This does not deduplicate
/// anything for types which support multiple values of the same key.
pub trait Combine {
    fn combine(&mut self, template: &Self);
}

/// Combine two map-like containers by recursively calling Merge on elements that
/// have the same key, and adding any missing key values from the template.
pub trait MergeCombine {
    fn merge_combine(&mut self, template: &Self);
}

/// Merge the elements in two containers using a custom function for getting the key
/// that should be used for each element. This allows merging Vec<T>.
pub trait MergeByKey {
    type Elem;

    fn merge_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq;
}

/// Combine and deduplicate items in a container that allows duplicates normally (such as Vec<T>)
/// while preserving the original element order.
pub trait CombineByKey {
    type Elem;

    fn combine_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq;
}

/// Merge and combine the elements in two containers using a custom function for getting
/// the key that should be used for each element. This allows merging and combining Vec<T>.
pub trait MergeCombineByKey {
    type Elem;

    fn merge_combine_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq;
}
