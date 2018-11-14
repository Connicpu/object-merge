use crate::Merge;
use std::time::Duration;

impl Merge for () {
    fn merge(&mut self, _template: &Self) {}
}

macro_rules! ignore_merge {
    ($($ty:ident)*) => {
        $(ignore_merge!(@ $ty);)*
    };
    (@ $ty:ident) => {
        impl Merge for $ty {
            fn merge(&mut self, _template: &Self) {}
        }
    };
}

ignore_merge!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 char bool);
ignore_merge!(String Duration);
