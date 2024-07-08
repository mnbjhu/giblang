use std::collections::HashMap;

use crate::parser::top::{func::Func, impl_::Impl, struct_::Struct, trait_::Trait};

pub enum ExportData {
    Func(Func),
    Struct(Struct),
    Trait(Trait, HashMap<String, Export>),
    Module(HashMap<String, Export>),
}

pub struct Export {
    pub data: ExportData,
    pub imports: Vec<QualifiedName>,
    pub references: Vec<QualifiedName>,
    pub impls: Vec<Impl>,
}

impl Export {
    pub fn new(data: ExportData) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
}

impl Default for Export {
    fn default() -> Self {
        Self {
            data: ExportData::Module(HashMap::new()),
            imports: vec![],
            references: vec![],
            impls: vec![],
        }
    }
}

pub type QualifiedName = Vec<String>;

impl Export {
    pub fn get(&self, key: &str) -> Option<&Export> {
        match &self.data {
            ExportData::Module(module) => module.get(key),
            _ => None,
        }
    }

    pub fn at_path(&self, path: impl Iterator<Item = String>) -> Option<&Export> {
        let mut current = self;
        for segment in path {
            current = if let ExportData::Module(module) = &current.data {
                module.get(&segment)?
            } else {
                return None;
            };
        }
        Some(current)
    }
}
