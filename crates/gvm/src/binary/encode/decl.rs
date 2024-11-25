use std::collections::HashMap;

use crate::format::func::FuncDef;

impl FuncDef {
    pub fn get_bytes(&self, id: u32) -> Vec<u8> {
        let mut bytes = vec![0];
        bytes.extend_from_slice(&id.to_be_bytes());
        bytes.extend_from_slice(&self.args.to_be_bytes());
        let len = self.name.len() as u32;
        bytes.extend_from_slice(&len.to_be_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.extend_from_slice(&self.pos.0.to_be_bytes());
        bytes.extend_from_slice(&self.pos.1.to_be_bytes());
        bytes.extend_from_slice(&self.file.to_be_bytes());

        let len = self.marks.len() as u32;
        bytes.extend_from_slice(&len.to_be_bytes());
        for (index, pos) in &self.marks {
            let index = *index as u32;
            bytes.extend_from_slice(&index.to_be_bytes());
            bytes.extend_from_slice(&pos.0.to_be_bytes());
            bytes.extend_from_slice(&pos.1.to_be_bytes());
        }

        for bc in &self.body {
            bytes.extend_from_slice(&bc.get_bytes());
        }
        bytes
    }
}

pub fn get_table_bytes(id: u64, items: &HashMap<u32, u32>) -> Vec<u8> {
    let mut bytes = vec![1];
    bytes.extend_from_slice(&id.to_be_bytes());
    let len: u32 = items.len() as u32;
    bytes.extend_from_slice(&len.to_be_bytes());
    for (k, v) in items {
        bytes.extend_from_slice(&k.to_be_bytes());
        bytes.extend_from_slice(&v.to_be_bytes());
    }
    bytes
}

pub fn get_file_name_bytes(id: u32, name: &str) -> Vec<u8> {
    let mut bytes = vec![49];
    bytes.extend_from_slice(&id.to_be_bytes());
    let len: u32 = name.len() as u32;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes
}
