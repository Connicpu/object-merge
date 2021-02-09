use crate::Overwrite;

impl<T> Overwrite for T
where
    T: Clone,
{
    fn overwrite(&mut self, template: &Self) {
        *self = template.clone()
    }
}
