use std::{
    cmp::Ordering,
    collections::HashMap,
    io::Stdout,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::run::{
    scope::Scope,
    state::{FuncDef, ProgramState},
    text::ByteCodeFile,
};
use dap::{
    events::{Event, StoppedEventBody},
    server::ServerOutput,
    types::StoppedEventReason,
};

pub struct Debugger<'code> {
    pub state: ProgramState<'code>,
    pub output: Arc<Mutex<ServerOutput<Stdout>>>,
    pub breakpoints: Arc<Mutex<HashMap<ByteCodeBreakpoint, i64>>>,
    pub paused: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByteCodeBreakpoint {
    pub func: u32,
    pub instr: usize,
}

impl<'db> Debugger<'db> {
    pub fn run_dap(&mut self) {
        let main = self.state.funcs.get(&0).expect("No main function");
        self.state.scopes.push(Scope::from_code(&main.body, 0));
        while *self.paused.lock().unwrap() {
            thread::sleep(Duration::from_millis(100));
        }
        while !self.state.scopes.is_empty() {
            let instr = self.state.next_instr();
            let point = ByteCodeBreakpoint {
                func: self.state.scope().id,
                instr: self.state.scope().index,
            };
            if let Some(bp) = self.breakpoints.lock().unwrap().get(&point) {
                self.output
                    .lock()
                    .unwrap()
                    .send_event(Event::Stopped(StoppedEventBody {
                        reason: StoppedEventReason::Breakpoint,
                        description: Some("Breakpoint hit desc".to_string()),
                        thread_id: None,
                        preserve_focus_hint: Some(false),
                        text: Some("Breakpoint hit".to_string()),
                        all_threads_stopped: Some(true),
                        hit_breakpoint_ids: Some(vec![*bp]),
                    }))
                    .unwrap();
                *self.paused.lock().unwrap() = true;
            }
            self.state.execute(instr);
        }
    }
}

impl ByteCodeFile {
    pub fn get_breakpoint(
        &self,
        file_name: &str,
        line: usize,
        col: usize,
    ) -> Option<ByteCodeBreakpoint> {
        let file_id = self
            .file_names
            .iter()
            .find_map(|(id, name)| if name == file_name { Some(id) } else { None })
            .unwrap_or_else(|| panic!("File not found: {file_name}, {:?}", self.file_names));
        self.get_func(*file_id, line, col).and_then(|func| {
            let instr = func.1.get_index(line, col);
            Some(ByteCodeBreakpoint {
                func: func.0,
                instr: instr?,
            })
        })
    }
    pub fn get_func(&self, file: u32, line: usize, col: usize) -> Option<(u32, &FuncDef)> {
        let mut instr = None;
        for (id, func) in &self.funcs {
            if func.file != file {
                continue;
            }
            let pos = (func.pos.0 as usize, func.pos.1 as usize);
            match cmp_pos(pos, (line, col)) {
                Ordering::Less => instr = Some((*id, func)),
                _ => {
                    if let Some(instr) = instr {
                        return Some(instr);
                    }
                }
            }
        }
        instr
    }
}

impl FuncDef {
    pub fn get_index(&self, line: usize, col: usize) -> Option<usize> {
        self.marks
            .iter()
            .rev()
            .find_map(|(offset, pos)| {
                let pos = (pos.0 as usize, pos.1 as usize);
                match cmp_pos(pos, (line, col)) {
                    Ordering::Less => Some(offset),
                    _ => None,
                }
            })
            .copied()
    }
}

pub fn cmp_pos(a: (usize, usize), b: (usize, usize)) -> std::cmp::Ordering {
    if a.0 < b.0 {
        Ordering::Less
    } else if a.0 > b.0 {
        Ordering::Greater
    } else if a.1 < b.1 {
        Ordering::Less
    } else if a.1 > b.1 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
