use broom::{trace::Trace, Handle};
use state::ProgramState;

pub mod bytecode;
pub mod state;
pub mod scope;

pub enum Object {
    Unit,
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
    Vec(u32, Vec<Handle<Object>>),
}

impl Trace<Self> for Object {
    fn trace(&self, tracer: &mut broom::prelude::Tracer<Self>) {
        if let Object::Vec(_, items) = self { items.trace(tracer) }
    }
}

pub trait Run {
    fn run(&self, state: &mut ProgramState) -> Handle<Object>;
}

pub trait DebugText {
    fn get_text(&self, state: &ProgramState) -> String;
}

impl DebugText for Handle<Object> {
    fn get_text(&self, state: &ProgramState) -> String {
        match state.heap.get(self).expect("Not found") {
            Object::Unit => "()".to_string(),
            Object::Int(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            Object::String(s) => s.to_string(),
            Object::Char(c) => c.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::Vec(id, fields) => {
                format!(
                    "({id}, {})",
                    fields
                        .iter()
                        .map(|it| it.get_text(state))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
