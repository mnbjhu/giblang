use std::collections::HashMap;

use crate::parser::{top::Top, File};

use super::export::{Export, ExportData};

pub struct Project {
    pub exports: Export,
    pub unresolved: HashMap<String, Vec<String>>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            exports: Export::default(),
            unresolved: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file: File, path: &[String]) {
        let module = self.exports.get_or_new_path(path.into_iter());
        for (item, _) in file {
            let name = item.name().to_string();
            let data = match item {
                Top::Impl(_) | Top::Use(_) => {
                    continue;
                }
                Top::Func(func) => ExportData::Func(func),
                Top::Struct(struct_) => ExportData::Struct(struct_),
                Top::Trait(mut trait_) => {
                    let mut funcs = HashMap::new();
                    while !trait_.body.is_empty() {
                        let func = trait_.body.pop().unwrap().0;
                        funcs.insert(func.name.0.clone(), Export::new(ExportData::Func(func)));
                    }
                    ExportData::Trait(trait_, funcs)
                }
            };
            module.insert(name, Export::new(data));
        }
    }
}
