use std::{collections::HashMap, process::exit};

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
                    let StackItem::Heap(dyn_) = self.pop() else {
                        panic!("Expected heap obj");
                    };
                    if i == 0 {
                        let HeapItem::Dyn(id, refr) = self.heap.get(dyn_).unwrap() else {
                            panic!("Expected dyn")
                        };
                        type_id = *id;
                        args.push(*refr);
                    } else {
                        args.push(self.pop());
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
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a + b);
                        self.push(res);
                        return;
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a + b);
                        self.push(res);
                        return;
                    }
                    (StackItem::Heap(a), StackItem::Heap(b)) => {
                        let a = self.heap.get(a).unwrap();
                        let b = self.heap.get(b).unwrap();
                        match (a, b) {
                            (HeapItem::Object(ai, ad), HeapItem::Object(bi, bd)) => {
                                assert_eq!(
                                    ai, bi,
                                    "Object types must match but found: {} and {}",
                                    ai, bi
                                );
                                let mut res = vec![];
                                res.extend(ad);
                                res.extend(bd);
                                let refr = self.heap.insert(HeapItem::Object(*ai, res));
                                self.push(StackItem::Heap(refr.into()));
                            }
                            (HeapItem::String(_), HeapItem::String(_)) => todo!(),
                            _ => {}
                        }
                    }
                    _ => {}
                }
                panic!("Can only add numbers, strings or vectors")
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
                let res = StackItem::Bool(a == b);
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
                let res = &data[*index as usize];
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
                if let StackItem::Heap(refr) = &refr {
                    let data = self.heap.get(refr).unwrap();
                    let refr = self.heap.insert(data.clone());
                    self.push(StackItem::Heap(refr.into()));
                } else {
                    self.push(refr);
                }
            }
            ByteCode::VecGet => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get(refr).unwrap() else {
                    panic!("Expected vec")
                };
                let res = data[index as usize];
                self.push(res);
            }
            ByteCode::VecSet => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let value = self.pop();
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get_mut(refr).unwrap() else {
                    panic!("Expected vec")
                };
                data[index as usize] = value;
            }
            ByteCode::VecPush => {
                let value = self.pop();
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get_mut(refr).unwrap() else {
                    panic!("Expected vec")
                };
                data.push(value);
            }
            ByteCode::VecPop => {
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get_mut(refr).unwrap() else {
                    panic!("Expected vec")
                };
                let res = data.pop().unwrap();
                self.push(res);
            }
            ByteCode::VecPeak => {
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get(refr).unwrap() else {
                    panic!("Expected vec")
                };
                let res = data.last().unwrap();
                self.push(*res);
            }
            ByteCode::VecInsert => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let value = self.pop();
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get_mut(refr).unwrap() else {
                    panic!("Expected vec")
                };
                data.insert(index as usize, value);
            }
            ByteCode::VecRemove => {
                let StackItem::Int(index) = self.pop() else {
                    panic!("Expected index to be an int")
                };
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get_mut(refr).unwrap() else {
                    panic!("Expected vec")
                };
                let res = data.remove(index as usize);
                self.push(res);
            }
            ByteCode::VecLen => {
                let StackItem::Heap(refr) = self.pop() else {
                    panic!("Expected heap obj")
                };
                let HeapItem::Object(_, data) = self.heap.get(refr).unwrap() else {
                    panic!("Expected vec")
                };
                let res = StackItem::Int(data.len() as i32);
                self.push(res);
            }
        };
    }
}
