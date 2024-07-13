use crate::fs::export::Export;

#[derive(Clone)]
pub enum Ty<'module> {
    Error,
    Named { name: Export<'module> },
    Generic { super_: Option<Box<Ty<'module>>> },
}
