use enum_::EnumIR;
use func::FuncIR;
use impl_::ImplIR;
use struct_::StructIR;
use trait_::TraitIR;

use crate::{
    check::{build_state::BuildState, err::CheckError, state::CheckState},
    db::input::Db,
    parser::top::Top,
    run::state::FuncDef,
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

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TopIR<'db> {
    Func(FuncIR<'db>),
    Struct(StructIR<'db>),
    Enum(EnumIR<'db>),
    Trait(TraitIR<'db>),
    Impl(ImplIR<'db>),
    Use(SpannedQualifiedNameIR<'db>),
}

impl<'db> Top {
    pub fn check(&self, state: &mut CheckState<'db>) -> TopIR<'db> {
        state.enter_scope();
        let name = self.get_name();
        if let Some(name) = name {
            state.enter_decl(name);
        }
        let ir = match &self {
            Top::Use(u) => {
                let name = state.get_ident_ir(u);
                let res = state.import(u);
                if let Err(e) = res {
                    state.error(CheckError::Unresolved(e));
                }
                TopIR::Use(name)
            }
            Top::Enum(e) => TopIR::Enum(e.check(state)),
            Top::Trait(t) => TopIR::Trait(t.check(state)),
            Top::Struct(s) => TopIR::Struct(s.check(state)),
            Top::Func(f) => TopIR::Func(f.check(state, false)),
            Top::Impl(i) => TopIR::Impl(i.check(state)),
        };
        if name.is_some() {
            state.exit_decl();
        }
        let _ = state.exit_scope();
        ir
    }
}

impl<'db> IrNode<'db> for TopIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        match self {
            TopIR::Func(f) => f.at_offset(offset, state),
            TopIR::Struct(s) => s.at_offset(offset, state),
            TopIR::Enum(e) => e.at_offset(offset, state),
            TopIR::Trait(t) => t.at_offset(offset, state),
            TopIR::Impl(i) => i.at_offset(offset, state),
            TopIR::Use(u) => u.at_offset(offset, state),
        }
    }

    fn tokens(&self, tokens: &mut Vec<crate::check::SemanticToken>, state: &mut IrState<'db>) {
        match self {
            TopIR::Func(f) => f.tokens(tokens, state),
            TopIR::Struct(s) => s.tokens(tokens, state),
            TopIR::Enum(e) => e.tokens(tokens, state),
            TopIR::Trait(t) => t.tokens(tokens, state),
            TopIR::Impl(i) => i.tokens(tokens, state),
            TopIR::Use(u) => u.tokens(tokens, state),
        }
    }
}

impl<'db> TopIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<(u32, FuncDef)> {
        // TODO: Add constructors for other types
        match self {
            TopIR::Func(f) => vec![f.build(state)],
            TopIR::Impl(i) => i.build(state),
            _ => vec![],
        }
    }
}
