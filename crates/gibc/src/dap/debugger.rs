use std::{
    cmp::Ordering,
    io::Stdout,
    sync::{Arc, Mutex},
};

use dap::{
    events::{Event, OutputEventBody, StoppedEventBody},
    server::ServerOutput,
    types::StoppedEventReason,
};
use dashmap::DashMap;
use gvm::{
    format::{func::FuncDef, ByteCodeFile},
    vm::{scope::Scope, state::ProgramState},
};

use super::DynResult;

pub struct Debugger<'code> {
    pub state: ProgramState<'code>,
    pub output: Arc<Mutex<ServerOutput<Stdout>>>,
    pub breakpoints: Arc<DashMap<ByteCodeBreakpoint, i64>>,
    pub paused: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ByteCodeBreakpoint {
    pub func: u32,
    pub instr: usize,
}

impl<'code> Debugger<'code> {
    pub fn new(
        state: ProgramState<'code>,
        output: Arc<Mutex<ServerOutput<Stdout>>>,
        breakpoints: Arc<DashMap<ByteCodeBreakpoint, i64>>,
        paused: Arc<Mutex<bool>>,
    ) -> Self {
        let mut res = Self {
            state,
            output,
            breakpoints,
            paused,
        };
        let main = res.state.funcs.get(&0).expect("No main function");
        res.state.scopes.push(Scope::from_code(&main.body, 0));
        res
    }

    pub fn poll(&mut self) -> DynResult<()> {
        if *self.paused.lock().unwrap() {
            return Ok(());
        }
        let instr = self.state.next_instr();
        let point = ByteCodeBreakpoint {
            func: self.state.scope().id,
            instr: self.state.scope().index,
        };
        if let Some(bp) = self.breakpoints.get(&point) {
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
                }))?;
            self.output
                .lock()
                .unwrap()
                .send_event(Event::Output(OutputEventBody {
                    output: format!("Hit breakpoint {:?}", bp.value()),
                    ..Default::default()
                }))?;
            *self.paused.lock().unwrap() = true;
        }
        self.state.execute(instr);
        Ok(())
    }
}

pub trait ByteCodeExt {
    fn get_breakpoint(
        &self,
        file_name: &str,
        line: usize,
        col: usize,
    ) -> Option<ByteCodeBreakpoint>;
    fn get_func(&self, file: u32, line: usize, col: usize) -> Option<(u32, &FuncDef)>;
}

impl ByteCodeExt for ByteCodeFile {
    fn get_breakpoint(
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
        self.get_func(*file_id, line, col).map(|func| {
            let instr = func.1.get_index(line, col);
            ByteCodeBreakpoint {
                func: func.0,
                instr,
            }
        })
    }
    fn get_func(&self, file: u32, line: usize, col: usize) -> Option<(u32, &FuncDef)> {
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

pub trait FuncDefExt {
    fn get_index(&self, line: usize, col: usize) -> usize;
}

impl FuncDefExt for FuncDef {
    fn get_index(&self, line: usize, col: usize) -> usize {
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
            .unwrap_or(0)
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
