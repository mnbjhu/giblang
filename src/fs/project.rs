use std::collections::HashMap;

use crate::parser::File;

use super::export::Export;

pub struct Project {
    pub exports: Export,
    pub unresolved: HashMap<String, Vec<String>>,
}

impl Project {
    pub fn insert(&mut self, _: File) {
        // TODO: Handles adding or updating a file
    }
}

pub struct CheckScope {
    // TODO: Should hold lightweight copies of types in scope for the export
}
