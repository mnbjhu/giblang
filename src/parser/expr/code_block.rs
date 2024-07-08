use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    lexer::token::{newline, punct},
    parser::{common::optional_newline::optional_newline, stmt::Stmt},
    util::Spanned,
    AstParser,
};

pub type CodeBlock = Vec<Spanned<Stmt>>;

pub fn code_block_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(CodeBlock) {
    stmt.map_with(|s, e| (s, e.span()))
        .separated_by(just(newline()))
        .collect()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
}

#[cfg(test)]
mod tests {
    use chumsky::{input::Input, Parser};

    use crate::{
        lexer::parser::lexer,
        parser::{
            common::pattern::Pattern,
            expr::{code_block::code_block_parser, Expr},
            stmt::{let_::LetStatement, stmt_parser, Stmt},
        },
        util::Span,
    };

    #[test]
    fn test_code_block_parser() {
        let input = r#"{
    let x = 1
    let y = 2
}"#;
        let tokens = lexer().parse(input).unwrap();
        let eoi = Span::splat(input.len());
        let input = tokens.spanned(eoi);
        let stmts = code_block_parser(stmt_parser()).parse(input).unwrap();
        assert_eq!(stmts.len(), 2);
        assert_eq!(
            stmts[0].0,
            Stmt::Let(LetStatement {
                pattern: (Pattern::Name("x".to_string()), (10..11).into()),
                value: (Expr::Literal(1.into()), (14..15).into()),
                ty: None,
            })
        );

        assert_eq!(
            stmts[1].0,
            Stmt::Let(LetStatement {
                pattern: (Pattern::Name("y".to_string()), (24..25).into()),
                value: (Expr::Literal(2.into()), (28..29).into()),
                ty: None,
            })
        );
    }
}
