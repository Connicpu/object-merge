use crate::{Combine, CombineByKey, Merge, MergeByKey, MergeCombineByKey};

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

impl<T> Combine for Vec<T>
where
    T: Clone,
{
    fn combine(&mut self, template: &Self) {
        self.extend_from_slice(template);
    }
}

fn merge_combine<'a, T, F, K>(
    this: &mut Vec<T>,
    template: &'a Vec<T>,
    mut get_key: F,
) -> HashMap<K, &'a T>
where
    T: Merge,
    F: FnMut(&T) -> K,
    K: Hash + Eq,
{
    let mut template: HashMap<K, &T> = template.iter().map(|item| (get_key(item), item)).collect();

    for item in this.iter_mut() {
        let template = {
            let key = get_key(item);
            match template.remove(&key) {
                Some(template) => template,
                None => continue,
            }
        };
        item.merge(&template);
    }

    template
}

impl<T> MergeByKey for Vec<T>
where
    T: Merge,
{
    type Elem = T;

    fn merge_by_key<F, K>(&mut self, template: &Self, get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq,
    {
        merge_combine(self, template, get_key);
    }
}

impl<T> CombineByKey for Vec<T>
where
    T: Clone,
{
    type Elem = T;

    fn combine_by_key<F, K>(&mut self, template: &Self, mut get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq,
    {
        self.extend_from_slice(template);
        let mut seen = HashSet::new();
        self.retain(|item| {
            let key = get_key(item);
            if seen.contains(&key) {
                false
            } else {
                seen.insert(key);
                true
            }
        });
    }
}

impl<T> MergeCombineByKey for Vec<T>
where
    T: Merge + Clone,
{
    type Elem = T;

    fn merge_combine_by_key<F, K>(&mut self, template: &Self, mut get_key: F)
    where
        F: FnMut(&Self::Elem) -> K,
        K: Hash + Eq,
    {
        let hash_template = merge_combine(self, template, &mut get_key);
        // We must preserve original order
        for value in template {
            if hash_template.contains_key(&get_key(value)) {
                self.push(value.clone());
            }
        }
    }
}
