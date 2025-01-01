use core::panic;
use std::{collections::HashMap, fs::read_to_string, io::stdin, process::exit};

use crate::{format::instr::ByteCode, vm::text::DebugText as _};

use super::{heap::HeapItem, scope::Scope, stack::StackItem, state::ProgramState};

#[allow(clippy::too_many_lines)]
impl<'code> ProgramState<'code> {
    pub fn execute(&mut self, code: &'code ByteCode) {
        match code {
            ByteCode::Push(lit) => {
                let lit = self.create(lit);
                self.push(lit);
            }
            ByteCode::Pop => {
                self.pop();
            }
            ByteCode::Match(expected) => {
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                if let HeapItem::Object(id, _) = self.heap.get(refr).unwrap() {
                    let res = StackItem::Bool(id == expected);
                    self.push(res);
                } else {
                    panic!("Expected vec")
                }
            }
            ByteCode::Copy => {
                let refr = self.peak();
                self.push(*refr);
            }
            ByteCode::Print => {
                print!("{}", self.pop().get_text(self));
            }
            ByteCode::Panic => {
                println!("{}", self.pop().get_text(self));
                println!("{}", self.stack_trace());
                exit(1);
            }
            ByteCode::Call(id) => {
                let func = &self.funcs[id];
                let mut args = Vec::with_capacity(func.args as usize);
                for _ in 0..func.args {
                    args.insert(0, self.pop());
                }
                let scope = Scope {
                    args,
                    locals: HashMap::new(),
                    stack: Vec::new(),
                    code: &func.body,
                    index: 0,
                    id: *id,
                };
                self.scopes.push(scope);
            }
            ByteCode::Dyn(id) => {
                let item = self.pop();
                let refr = self.heap.insert(HeapItem::Dyn(*id, item));
                let res = StackItem::Heap(refr.into());
                self.push(res);
            }
            ByteCode::DynCall(func_id) => {
                let trait_func = &self.funcs[func_id];
                let mut args = Vec::with_capacity(trait_func.args as usize);
                let mut type_id = 0;
                for i in (0..trait_func.args).rev() {
                    let next = self.pop();
                    if i == 0 {
                        let StackItem::Heap(dyn_) = next else {
                            panic!(
                                "Expected heap obj but found {next:?} \n{}",
                                self.stack_trace()
                            );
                        };
                        let HeapItem::Dyn(id, refr) = self.heap.get(dyn_).unwrap() else {
                            panic!("Expected dyn")
                        };
                        type_id = *id;
                        args.insert(0, *refr);
                    } else {
                        args.push(next);
                    }
                }
                let impl_func = self.get_trait_impl(*func_id, type_id).unwrap();
                let code = &self.funcs[&impl_func].body;
                let scope = Scope {
                    args,
                    locals: HashMap::new(),
                    stack: Vec::new(),
                    code,
                    index: 0,
                    id: impl_func,
                };
                self.scopes.push(scope);
            }
            ByteCode::Return => {
                let ret = self.scope_mut().stack.pop();
                self.scopes.pop();
                if let Some(ret) = ret {
                    if !self.scopes.is_empty() {
                        self.push(ret);
                    }
                }
            }
            ByteCode::Construct { id, len } => {
                let mut args = Vec::new();
                for _ in 0..*len {
                    args.push(self.pop());
                }
                let refr = self.heap.insert(HeapItem::Object(*id, args));
                self.push(StackItem::Heap(refr.into()));
            }
            ByteCode::NewLocal(id) => {
                let refr = self.pop();
                self.new_local(*id, refr);
            }
            ByteCode::GetLocal(id) => {
                let local = self.get_local(*id);
                self.push(local);
            }
            ByteCode::SetLocal(id) => {
                let refr = self.pop();
                self.set_local(*id, refr);
            }
            ByteCode::Je(line) => {
                if let StackItem::Bool(cond) = self.pop() {
                    if cond {
                        self.scope_mut().index = *line as usize;
                    }
                } else {
                    panic!("Expected condition to be a boolean")
                }
            }
            ByteCode::Jne(line) => {
                if let StackItem::Bool(cond) = self.pop() {
                    if !cond {
                        self.scope_mut().index = *line as usize;
                    }
                } else {
                    panic!("Expected condition to be a boolean")
                }
            }
            ByteCode::Jmp(line) => {
                self.scope_mut().index = *line as usize;
            }
            ByteCode::Param(id) => {
                let refr = self.get_param(*id);
                self.push(refr);
            }
            ByteCode::Mod => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a % b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'mod' non-int")
                    }
                }
            }
            ByteCode::Mul => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a * b);
                        self.push(res);
                    }
                    (StackItem::Int(a), StackItem::Float(b)) => {
                        let res = StackItem::Float((a as f32) * b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Int(b)) => {
                        let res = StackItem::Float(a * (b as f32));
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a * b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'mul' non-numbers")
                    }
                }
            }
            ByteCode::Div => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a / b);
                        self.push(res);
                    }
                    (StackItem::Int(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a as f32 / b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Int(b)) => {
                        let res = StackItem::Float(a / (b as f32));
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a / b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'div' non-numbers")
                    }
                }
            }
            ByteCode::Add => {
                let b = self.pop();
                let a = self.pop();
                let res = a.add(&b, self);
                self.push(res)
            }
            ByteCode::Sub => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a - b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a - b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'sub' non-numbers")
                    }
                }
            }
            ByteCode::And => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Bool(a), StackItem::Bool(b)) => {
                        let res = StackItem::Bool(a && b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'and' non-bools")
                    }
                }
            }
            ByteCode::Or => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Bool(a), StackItem::Bool(b)) => {
                        let res = StackItem::Bool(a || b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'or' non-bools")
                    }
                }
            }
            ByteCode::Eq => {
                let b = self.pop();
                let a = self.pop();
                let res = StackItem::Bool(a.is_equal(&b, self));
                self.push(res);
            }
            ByteCode::Neq => {
                let b = self.pop();
                let a = self.pop();
                let res = StackItem::Bool(a != b);
                self.push(res);
            }
            ByteCode::Lt => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a < b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a < b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '<' non-numbers")
                    }
                }
            }
            ByteCode::Gt => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a > b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a > b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '>' non-numbers")
                    }
                }
            }
            ByteCode::Lte => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a <= b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a <= b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '<=' non-numbers")
                    }
                }
            }
            ByteCode::Gte => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a >= b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a >= b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '>=' non-numbers")
                    }
                }
            }
            ByteCode::Not => match self.pop() {
                StackItem::Bool(a) => {
                    let res = StackItem::Bool(!a);
                    self.push(res);
                }
                _ => {
                    panic!("Cannot 'or' non-bools")
                }
            },
            ByteCode::Index(index) => {
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get(refr).unwrap() else {
                    panic!("Expected vec")
                };
                let res = data.get(*index as usize).unwrap_or_else(|| {
                    panic!(
                        "Index {} is out of range for obj of size {}\n{}",
                        index,
                        data.len(),
                        self.stack_trace()
                    )
                });
                self.push(*res);
            }
            ByteCode::SetIndex(index) => {
                let value = self.pop();
                let vec = self.pop();
                let StackItem::Heap(refr) = &vec else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get_mut(refr).unwrap() else {
                    panic!("Expected vec")
                };
                data[*index as usize] = value;
            }
            ByteCode::Clone => {
                let refr = self.pop();
                let res = refr.do_clone(self);
                self.push(res)
            }
            ByteCode::VecGet => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let index = index.try_into().unwrap();
                let res = self.pop().get(index, self).unwrap();
                self.push(res);
            }
            ByteCode::VecSet => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let value = self.pop();
                self.pop().set(index as u32, value, self);
            }
            ByteCode::VecPush => {
                let value = self.pop();
                self.pop().push(value, self);
            }
            ByteCode::VecPop => {
                let res = self.pop().pop(self).expect("Cannot pop from empty vec");
                self.push(res)
            }
            ByteCode::VecPeak => {
                let res = self.pop().peak(self).unwrap();
                self.push(res);
            }
            ByteCode::VecInsert => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let value = self.pop();
                self.pop().insert(index as u32, value, self);
            }
            ByteCode::VecRemove => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let res = self.pop().remove(index as u32, self).unwrap();
                self.push(res);
            }
            ByteCode::VecLen => {
                let len = self.pop().len(self).try_into().unwrap();
                self.push(StackItem::Int(len));
            }
            ByteCode::Slice => {
                let StackItem::Int(end) = self.pop() else {
                    panic!("Expected end to be an int")
                };
                let StackItem::Int(start) = self.pop() else {
                    panic!("Expected start to be an int")
                };
                let res = self.pop().slice(start as u32, end as u32, self);
                self.push(res)
            }
            ByteCode::ReadLine => {
                let mut buf = String::new();
                stdin().read_line(&mut buf).unwrap();
                let data = HeapItem::String(buf);
                let refr = self.heap.insert(data);
                self.push(StackItem::Heap(refr.into()));
            }
            ByteCode::ReadFile => {
                let Some(path) = self.pop().string_data(self) else {
                    panic!("Expected a filename")
                };
                let text = read_to_string(path).unwrap();
                let data = HeapItem::String(text);
                let refr = self.heap.insert(data);
                self.push(StackItem::Heap(refr.into()));
            }
        };
    }
}
