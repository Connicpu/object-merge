use crate::{Merge, MergeByKey};

use std::collections::HashMap;
use std::hash::Hash;

fn merge_combine<'a, T, F, K, const N: usize>(
    this: &mut [T; N],
    template: &'a [T],
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
        item.merge(template);
    }

    template
}

impl<T, const N: usize> Merge for [T; N]
where
    T: Merge,
{
    fn merge(&mut self, template: &Self) {
        for (x, y) in std::iter::zip(self, template) {
            x.merge(y)
        }
    }
}

impl<T, const N: usize> MergeByKey for [T; N]
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
