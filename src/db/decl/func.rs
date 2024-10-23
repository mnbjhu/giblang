use salsa::Update;

use crate::ty::{Generic, Ty};


#[derive(Update, Debug, Clone, PartialEq)]
pub struct Function<'db> {
    pub name: String,
    pub generics: Vec<Generic<'db>>,
    pub receiver: Option<Ty<'db>>,
    pub args: Vec<(String, Ty<'db>)>,
    pub ret: Ty<'db>,
    pub required: bool,
}

