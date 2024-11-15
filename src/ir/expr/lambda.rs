use std::collections::HashMap;

use crate::{
    check::{
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    ir::{
        common::pattern::PatternIR,
        ty::{TypeIR, TypeIRData},
        ContainsOffset, IrNode, IrState,
    },
    parser::expr::lambda::{Lambda, LambdaParam},
    ty::{FuncTy, Generic, Ty},
    util::{Span, Spanned},
};

use super::{
    block::{check_block, expect_block, CodeBlockIR},
    ExprIR, ExprIRData,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct LambdaIR<'db> {
    pub args: Vec<Spanned<LambdaParamIR<'db>>>,
    pub body: Spanned<CodeBlockIR<'db>>,
    pub vars: HashMap<String, VarDecl<'db>>,
    pub generics: HashMap<String, Generic<'db>>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct LambdaParamIR<'db> {
    pub pattern: Spanned<PatternIR<'db>>,
    pub ty: Spanned<TypeIR<'db>>,
}
impl<'db> Lambda {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        state.enter_scope();
        let mut args = vec![];
        for (arg, span) in &self.args {
            args.push((arg.check(state, *span), *span));
        }

        let ExprIR {
            data: ExprIRData::CodeBlock(body),
            ty,
        } = check_block(&self.body.0, state)
        else {
            panic!("Expected block");
        };
        let (vars, generics) = state.exit_scope();
        let ty = Ty::Function(FuncTy {
            receiver: None,
            args: args.iter().map(|arg| arg.0.ty.0.ty.clone()).collect(),
            ret: Box::new(ty),
        });
        let ir = LambdaIR {
            args,
            body: (body, self.body.1),
            vars,
            generics,
        };
        ExprIR {
            data: ExprIRData::Lambda(ir),
            ty,
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        expected: &Ty<'db>,
        span: Span,
    ) -> ExprIR<'db> {
        state.enter_scope();
        if let Ty::Function(expected) = expected {
            let args = self
                .args
                .iter()
                .zip(&expected.args)
                .map(|((arg, span), expected)| (arg.expect(state, expected, *span), *span))
                .collect();
            if self.args.len() > expected.args.len() {
                for arg in self.args.iter().skip(expected.args.len()) {
                    state.simple_error("Unexpected argument", arg.1);
                }
            }
            if self.args.is_empty() && expected.args.len() == 1 {
                state.insert_variable(
                    "it".to_string(),
                    expected.args[0].clone(),
                    TokenKind::Var,
                    span,
                );
            }
            if let Some(receiver) = &expected.receiver {
                state.add_self_param(receiver.as_ref().clone(), span);
            }
            let ExprIR {
                data: ExprIRData::CodeBlock(body),
                ty,
            } = expect_block(&self.body.0, state, &expected.ret, self.body.1)
            else {
                panic!("Expected block???");
            };
            let (vars, generics) = state.exit_scope();
            ExprIR {
                data: ExprIRData::Lambda(LambdaIR {
                    args,
                    body: (body, self.body.1),
                    vars,
                    generics,
                }),
                ty,
            }
        } else {
            let ir = self.check(state);
            let ty = ir.ty.clone();
            ty.expect_is_instance_of(expected, state, span);
            let (vars, generics) = state.exit_scope();
            let ExprIR {
                data: ExprIRData::CodeBlock(body),
                ty,
            } = check_block(&self.body.0, state)
            else {
                panic!("Expected a code block???");
            };
            let ir = LambdaIR {
                args: self
                    .args
                    .iter()
                    .map(|(arg, span)| (arg.check(state, *span), *span))
                    .collect(),
                body: (body, self.body.1),
                vars,
                generics,
            };
            ExprIR {
                data: ExprIRData::Lambda(ir),
                ty,
            }
        }
    }
}

impl<'db> LambdaParam {
    pub fn check(&self, state: &mut CheckState<'db>, span: Span) -> LambdaParamIR<'db> {
        if let Some(ty) = &self.ty {
            let expected = ty.0.check(state);
            let pattern = (self.pattern.0.expect(state, &expected.ty), self.pattern.1);
            LambdaParamIR {
                pattern,
                ty: (expected, ty.1),
            }
        } else {
            let id = state
                .type_state
                .new_type_var(self.pattern.1, state.file_data);
            let type_var = Ty::TypeVar { id };
            let pattern = (self.pattern.0.expect(state, &type_var), self.pattern.1);

            LambdaParamIR {
                pattern,
                ty: (
                    TypeIR {
                        data: TypeIRData::Wildcard(span),
                        ty: type_var,
                    },
                    span,
                ),
            }
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        expected: &Ty<'db>,
        span: Span,
    ) -> LambdaParamIR<'db> {
        let ir = self.check(state, span);
        ir.ty.0.ty.expect_is_instance_of(expected, state, ir.ty.1);
        ir
    }
}

impl<'db> IrNode<'db> for LambdaIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        for (arg, span) in &self.args {
            if span.contains_offset(offset) {
                return arg;
            }
        }
        if self.body.1.contains_offset(offset) {
            return self.body.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        for arg in &self.args {
            arg.0.tokens(tokens, state);
        }
        self.body.0.tokens(tokens, state);
    }
}

impl<'db> IrNode<'db> for LambdaParamIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        self.pattern.0.tokens(tokens, state);
        self.ty.0.tokens(tokens, state);
    }
}
