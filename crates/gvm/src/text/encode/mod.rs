use std::fmt::{Display, Formatter};

use decl::{write_file_name, write_func_def, write_table};

use crate::format::ByteCodeFile;

mod decl;
mod instr;
mod literal;

impl Display for ByteCodeFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, name) in &self.file_names {
            write_file_name(*id, name, f)?;
        }
        for (id, table) in &self.tables {
            write_table(*id, table, f)?;
        }
        for (id, func) in &self.funcs {
            write_func_def(*id, func, f)?;
        }
        Ok(())
    }
}
