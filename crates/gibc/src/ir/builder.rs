use chumsky::span::Span as _;
use gvm::format::instr::ByteCode;

use crate::{range::offset_to_position_str, util::Span};

pub enum ByteCodeNode {
    Code(Vec<ByteCode>),
    Block(Vec<ByteCodeNode>),
    If {
        branches: Vec<(Box<ByteCodeNode>, Box<ByteCodeNode>)>,
        else_: Option<Box<ByteCodeNode>>,
    },
    While(Box<ByteCodeNode>, Box<ByteCodeNode>),
    Spanned(Box<ByteCodeNode>, Span),
    MaybeBreak,
    Continue,
    Break,
    Next,
}

impl ByteCodeNode {
    pub fn build(
        self,
        top: u32,
        break_: u32,
        continue_: u32,
        next: u32,
        marks: &mut Vec<(usize, (u16, u16))>,
        text: &str,
    ) -> Vec<ByteCode> {
        let len = self.len();
        match self {
            ByteCodeNode::Code(code) => code,
            ByteCodeNode::Block(block) => {
                let mut found = vec![];
                let mut top = top;
                for stmt in block {
                    let code = stmt.build(top, break_, continue_, next, marks, text);
                    top += code.len() as u32;
                    found.extend(code);
                }
                found
            }
            ByteCodeNode::If { branches, else_ } => {
                let mut top = top;
                let mut found = vec![];
                let end = top + len;
                let last = branches.len() - 1;
                for (index, (cond, then)) in branches.into_iter().enumerate() {
                    let is_last_branch = index == last && else_.is_none();
                    let next = if is_last_branch {
                        end
                    } else {
                        top + cond.len() + then.len() + 1
                    };
                    let mut code = cond.build(top, break_, continue_, next, marks, text);
                    top += code.len() as u32;
                    let body = then.build(top, break_, continue_, next, marks, text);
                    top += body.len() as u32;
                    code.extend(body);
                    if !is_last_branch {
                        code.push(ByteCode::Jmp(end));
                        top += 1;
                    }
                    found.extend(code);
                }
                if let Some(else_) = else_ {
                    found.extend(else_.build(top, break_, continue_, next, marks, text));
                }
                found
            }
            ByteCodeNode::While(cond, then) => {
                let new_break = top + len;
                let mut code = cond.build(top, new_break, top, new_break, marks, text);
                code.extend(then.build(top + code.len() as u32, new_break, top, next, marks, text));
                code
            }
            ByteCodeNode::Break => vec![ByteCode::Jmp(break_)],
            ByteCodeNode::MaybeBreak => vec![ByteCode::Jne(break_)],
            ByteCodeNode::Continue => vec![ByteCode::Jmp(continue_)],
            ByteCodeNode::Next => vec![ByteCode::Jne(next)],
            ByteCodeNode::Spanned(inner, span) => {
                let code = inner.build(top, break_, continue_, next, marks, text);
                let pos = offset_to_position_str(span.start(), text);
                marks.push((top as usize, (pos.line as u16 + 1, pos.character as u16)));
                code
            }
        }
    }

    pub fn len(&self) -> u32 {
        match self {
            ByteCodeNode::Code(code) => code.len() as u32,
            ByteCodeNode::Block(block) => block.iter().map(ByteCodeNode::len).sum(),
            ByteCodeNode::If { branches, else_ } => {
                let mut len = 0;
                for (cond, then) in branches {
                    len += cond.len() + then.len() + 1;
                }
                if let Some(else_) = &else_ {
                    len + else_.len()
                } else {
                    len - 1
                }
            }
            ByteCodeNode::While(cond, then) => cond.len() + then.len(),
            ByteCodeNode::Break
            | ByteCodeNode::Next
            | ByteCodeNode::MaybeBreak
            | ByteCodeNode::Continue => 1,
            ByteCodeNode::Spanned(inner, _) => inner.len(),
        }
    }
}
