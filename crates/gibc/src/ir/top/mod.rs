use enum_::EnumIR;
use func::FuncIR;
use gvm::format::func::FuncDef;
use impl_::ImplIR;
use struct_::StructIR;
use trait_::TraitIR;

use crate::{
    check::{
        build_state::BuildState,
        err::CheckError,
        scoped_state::{Scope, Scoped},
        state::CheckState,
        SemanticToken,
    },
    parser::top::Top,
};

use super::{common::pattern::SpannedQualifiedNameIR, IrNode, IrState};

pub mod arg;
pub mod enum_;
pub mod enum_member;
pub mod func;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod struct_field;
pub mod trait_;

#[derive(Debug, PartialEq, Clone)]
pub enum TopIRData<'db> {
    Func(FuncIR<'db>),
    Struct(StructIR<'db>),
    Enum(EnumIR<'db>),
    Trait(TraitIR<'db>),
    Impl(ImplIR<'db>),
    Use(SpannedQualifiedNameIR<'db>),
}

impl<'db> Top {
    pub fn check(&self, state: &mut CheckState<'db>) -> TopIR<'db> {
        if let Top::Use(u) = self {
            let name = state.get_ident_ir(u);
            let res = state.import(u);
            if let Err(e) = res {
                state.error(CheckError::Unresolved(e));
            }
            return TopIR {
                data: TopIRData::Use(name),
                scope: Scope::default(),
            };
        }
        state.enter_scope();
        let name = self.get_name();
        if let Some(name) = name {
            state.enter_decl(name);
        }
        let data = match &self {
            Top::Enum(e) => TopIRData::Enum(e.check(state)),
            Top::Trait(t) => TopIRData::Trait(t.check(state)),
            Top::Struct(s) => TopIRData::Struct(s.check(state)),
            Top::Func(f) => TopIRData::Func(f.check(state, false)),
            Top::Impl(i) => TopIRData::Impl(i.check(state)),
            Top::Use(_) => unreachable!(),
        };
        if name.is_some() {
            state.exit_decl();
        }
        let scope = state.exit_scope();
        TopIR { data, scope }
    }
}

#[derive(Debug)]
pub struct TopIR<'db> {
    pub data: TopIRData<'db>,
    pub scope: Scope<'db>,
}

impl<'db> IrNode<'db> for TopIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        state.push_scope(self.scope.clone());
        let res = match &self.data {
            TopIRData::Func(f) => f.at_offset(offset, state),
            TopIRData::Struct(s) => s.at_offset(offset, state),
            TopIRData::Enum(e) => e.at_offset(offset, state),
            TopIRData::Trait(t) => t.at_offset(offset, state),
            TopIRData::Impl(i) => i.at_offset(offset, state),
            TopIRData::Use(u) => u.at_offset(offset, state),
        };
        res
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        match &self.data {
            TopIRData::Func(f) => f.tokens(tokens, state),
            TopIRData::Struct(s) => s.tokens(tokens, state),
            TopIRData::Enum(e) => e.tokens(tokens, state),
            TopIRData::Trait(t) => t.tokens(tokens, state),
            TopIRData::Impl(i) => i.tokens(tokens, state),
            TopIRData::Use(u) => u.tokens(tokens, state),
        }
    }

    fn debug_name(&self) -> &'static str {
        "TopIR"
    }
}

impl<'db> TopIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<(u32, FuncDef)> {
        // TODO: Add constructors for other types
        match &self.data {
            TopIRData::Func(f) => vec![f.build(state)],
            TopIRData::Impl(i) => i.build(state),
            TopIRData::Trait(t) => t.build(state),
            _ => vec![],
        }
    }
}
