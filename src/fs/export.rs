use std::collections::HashMap;

use ptree::TreeBuilder;

use crate::parser::top::{func::Func, impl_::Impl, struct_::Struct, trait_::Trait};

use super::diag::Diag;

pub enum ExportData {
    Func(Func),
    Struct(Struct),
    Trait(Trait, HashMap<String, Export>),
    Module(HashMap<String, Export>),
}

impl ExportData {
    pub fn name(&self) -> &'static str {
        match self {
            ExportData::Func(_) => "Func",
            ExportData::Struct(_) => "Struct",
            ExportData::Trait(_, _) => "Trait",
            ExportData::Module(_) => "Module",
        }
    }
}

pub struct Export {
    pub data: ExportData,
    pub imports: Vec<QualifiedName>,
    pub references: Vec<QualifiedName>,
    pub impls: Vec<Impl>,
    pub diags: Vec<Diag>,
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
            diags: vec![],
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

    pub fn get_or_new(&mut self, key: String) -> &mut Export {
        if self.contains_key(&key) {
            self.get_mut(&key).unwrap()
        } else {
            self.insert(key.to_string(), Export::default())
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match &self.data {
            ExportData::Trait(_, v) => v.contains_key(key),
            ExportData::Module(v) => v.contains_key(key),
            _ => false,
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

    pub fn get_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut Export> {
        match &mut self.data {
            ExportData::Module(module) => module.get_mut(key),
            _ => None,
        }
    }

    pub fn get_path_mut(&mut self, path: impl Iterator<Item = String>) -> Option<&mut Export> {
        let mut current = self;
        for segment in path {
            current = current.get_mut(&segment)?;
        }
        Some(current)
    }

    pub fn get_or_new_path<'a>(&mut self, path: impl Iterator<Item = &'a String>) -> &mut Export {
        let mut current = self;
        for seg in path {
            current = current.get_or_new(seg.clone());
        }
        current
    }

    pub fn insert(&mut self, key: String, export: Export) -> &mut Export {
        match (&mut self.data, &export.data) {
            (ExportData::Module(m), _) => {
                m.insert(key.to_string(), export);
                m.get_mut(&key).unwrap()
            }
            (ExportData::Trait(_, f), ExportData::Func(_)) => {
                f.insert(key.to_string(), export);
                f.get_mut(&key).unwrap()
            }
            _ => panic!("Cannot have {} as child module", export.data.name()),
        }
    }

    pub fn build_tree(&self, tree: &mut TreeBuilder, name: String) {
        match &self.data {
            ExportData::Func(_) => {
                tree.add_empty_child(format!("fn {}", name));
            }
            ExportData::Struct(_) => {
                tree.add_empty_child(format!("struct {}", name));
            }
            ExportData::Trait(_, fs) => {
                tree.begin_child(name);
                for name in fs.keys() {
                    tree.add_empty_child(format!("fn {}", name));
                }
                tree.end_child();
            }
            ExportData::Module(m) => {
                tree.begin_child(name);
                for (name, module) in m {
                    module.build_tree(tree, name.to_string())
                }
                tree.end_child();
            }
        };
    }
}
