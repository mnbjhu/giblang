use chumsky::primitive::choice;
use chumsky::Parser;

use crate::AstParser;

use super::member::{CallAccess, MemberCall};
use super::property::{Property, PropertyAccess};
use super::{member::member_call_parser, property::property_parser, Expr};

pub fn access_parser<'tokens, 'src: 'tokens>(
    atom: AstParser!(Expr),
    expr: AstParser!(Expr),
) -> AstParser!(Expr) {
    let access = choice((
        member_call_parser(expr.clone()).map(Access::MemberCall),
        property_parser().map(Access::Property),
    ));
    atom.foldl_with(access.repeated(), |a, b, e| match b {
        Access::MemberCall(call) => Expr::MemberCall(MemberCall {
            rec: (Box::new(a), e.span()),
            name: call.name,
            args: call.args,
        }),
        Access::Property(prop) => Expr::Property(Property {
            expr: (Box::new(a), e.span()),
            name: prop.name,
        }),
    })
}

#[derive(Clone, PartialEq, Debug)]
pub enum Access {
    MemberCall(CallAccess),
    Property(PropertyAccess),
}
