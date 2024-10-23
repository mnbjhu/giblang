use super::input::Db;

#[salsa::interned]
pub struct ModulePath<'db> {
    #[return_ref]
    pub name: Vec<String>,
}

impl<'db> ModulePath<'db> {
    #[must_use]
    pub fn get_parent(self, db: &'db dyn Db) -> ModulePath<'db> {
        let path = self.name(db);
        ModulePath::new(db, path[0..path.len() - 1].to_vec())
    }
}
