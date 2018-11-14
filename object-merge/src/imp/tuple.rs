use crate::Merge;

macro_rules! merge_tuple {
    () => {};
    ($($t:ident $i:tt),*) => {
        impl<$($t,)*> Merge for ($($t,)*)
        where
            $($t: Merge,)*
        {
            fn merge(&mut self, template: &Self) {
                $(Merge::merge(&mut self.$i, &template.$i);)*
            }
        }
    };
}

merge_tuple!(A 0);
merge_tuple!(A 0, B 1);
merge_tuple!(A 0, B 1, C 2);
merge_tuple!(A 0, B 1, C 2, D 3);
merge_tuple!(A 0, B 1, C 2, D 3, E 4);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
merge_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
