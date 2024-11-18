use broom::{trace::Trace, Handle};
use state::ProgramState;

pub mod bytecode;
pub mod debug;
pub mod scope;
pub mod state;
pub mod text;

#[derive(PartialEq, Debug, Clone)]
pub enum StackItem {
    Unit,
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
    Dyn(u32, Box<StackItem>),
    Vec(u32, Handle<Object>),
}

#[derive(PartialEq, Debug)]
pub struct Object(Vec<StackItem>);

impl Trace<Self> for Object {
    fn trace(&self, tracer: &mut broom::prelude::Tracer<Self>) {
        for item in &self.0 {
            if let StackItem::Vec(_, data) = item {
                data.trace(tracer)
            } else if let StackItem::Dyn(_, data) = item {
                if let StackItem::Vec(_, data) = &**data {
                    data.trace(tracer)
                }
            }
        }
    }
}

pub trait DebugText {
    fn get_text(&self, state: &ProgramState) -> String;
}

impl DebugText for StackItem {
    fn get_text(&self, state: &ProgramState) -> String {
        match self {
            StackItem::Unit => "()".to_string(),
            StackItem::Int(i) => i.to_string(),
            StackItem::Float(f) => f.to_string(),
            StackItem::String(s) => s.to_string(),
            StackItem::Char(c) => c.to_string(),
            StackItem::Bool(b) => b.to_string(),
            StackItem::Vec(id, fields) => {
                let fields = state.heap.get(fields).unwrap();
                format!(
                    "{id}:[{}]",
                    fields
                        .0
                        .iter()
                        .map(|it| it.get_text(state))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            StackItem::Dyn(id, data) => {
                format!("{id}:{{{}}}", data.get_text(state))
            }
        }
    }
}
