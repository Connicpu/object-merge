use crate::{Combine, Merge, MergeCombine};

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

impl<T> Combine for HashSet<T>
where
    T: Clone + Eq + Hash,
{
    fn combine(&mut self, template: &Self) {
        for item in template.iter() {
            if !self.contains(item) {
                self.insert(item.clone());
            }
        }
    }
}

impl<K, V> Merge for HashMap<K, V>
where
    K: Eq + Hash,
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

impl<K, V> Combine for HashMap<K, V>
where
    K: Clone + Eq + Hash,
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

impl<K, V> MergeCombine for HashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + Merge,
{
    fn merge_combine(&mut self, template: &Self) {
        let mut template: HashMap<&K, &V> = template.iter().collect();
        for (key, item) in self.iter_mut() {
            if let Some(template) = template.remove(key) {
                item.merge(template);
            }
        }
        self.extend(template.iter().map(|(&k, &v)| (k.clone(), v.clone())));
    }
}
