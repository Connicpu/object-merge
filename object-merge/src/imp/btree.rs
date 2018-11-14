use crate::{Combine, Merge, MergeCombine};

use std::collections::{BTreeMap, BTreeSet};

impl<T> Combine for BTreeSet<T>
where
    T: Ord + Clone,
{
    fn combine(&mut self, template: &Self) {
        for item in template.iter() {
            if !self.contains(item) {
                self.insert(item.clone());
            }
        }
    }
}

impl<K, V> Merge for BTreeMap<K, V>
where
    K: Ord,
    V: Merge,
{
    fn merge(&mut self, template: &Self) {
        for (key, item) in self.iter_mut() {
            if let Some(template) = template.get(key) {
                item.merge(template);
            }
        }
    }
}

impl<K, V> Combine for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn combine(&mut self, template: &Self) {
        for (key, value) in template.iter() {
            if !self.contains_key(key) {
                self.insert(key.clone(), value.clone());
            }
        }
    }
}

impl<K, V> MergeCombine for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: Clone + Merge,
{
    fn merge_combine(&mut self, template: &Self) {
        let mut template: BTreeMap<&K, &V> = template.iter().collect();
        for (key, item) in self.iter_mut() {
            if let Some(template) = template.remove(key) {
                item.merge(template);
            }
        }
        self.extend(template.iter().map(|(&k, &v)| (k.clone(), v.clone())));
    }
}
