#[macro_use]
extern crate derive_object_merge;
extern crate object_merge;

#[derive(Merge)]
pub struct Config {
    pub foo: Option<i32>,
    pub bar: Option<String>,

    #[combine_by(member(name))]
    pub providers: Option<Vec<Provider>>,
}

#[derive(Merge, Clone)]
pub struct Provider {
    pub name: String,
    pub address: String,
    pub email: String,
    pub phone: String,
}
