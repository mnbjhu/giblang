use std::fmt::Formatter;

use crate::format::{func::FuncDef, table::VTable};

pub fn write_func_def(id: u32, func: &FuncDef, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(
        f,
        "func {id} {args} \"{name}\" {line} {col} {file}",
        args = func.args,
        name = func.name,
        line = func.pos.0,
        col = func.pos.1,
        file = func.file,
    )?;
    for instr in &func.body {
        writeln!(f, "{}", instr)?;
    }
    Ok(())
}

pub fn write_table(id: u64, table: &VTable, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "type {id}")?;
    for (key, value) in table {
        writeln!(f, "    {key} {value}")?;
    }
    Ok(())
}

pub fn write_file_name(id: u32, name: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "file {id} {name}")
}
