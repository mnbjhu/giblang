use chumsky::{error::Rich, primitive::just, IterParser, Parser};

use crate::{lexer::token::newline, util::Spanned, AstParser};

use self::{
    common::optional_newline::optional_newline,
    top::{top_parser, Top},
};

pub mod common;
pub mod expr;
pub mod stmt;
pub mod top;

pub type File = Vec<Spanned<Top>>;

pub fn file_parser<'tokens, 'src: 'tokens>() -> AstParser!(File) {
    top_parser()
        .map_with(|t, e| (t, e.span()))
        .separated_by(just(newline()))
        .collect()
        .padded_by(optional_newline())
        .validate(|v: File, _, emitter| {
            let mut existing = vec![];
            for top in v.iter() {
                let name = top.0.get_name();
                if existing.contains(&name) {
                    emitter.emit(Rich::custom(
                        top.0.name_span(),
                        format!("Duplicate top-level item '{}'", name.unwrap()),
                    ))
                } else {
                    existing.push(name);
                }
            }
            v
        })
}

#[macro_export]
macro_rules! assert_parse_eq {
    ($parser:expr, $input:expr, $expected:expr) => {
        #[allow(unused_imports)]
        use chumsky::input::Input as _;
        #[allow(unused_imports)]
        use chumsky::Parser as _;
        let tokens = $crate::lexer::parser::lexer().parse($input).unwrap();
        let eoi = $crate::util::Span::splat($input.len());
        let input = tokens.spanned(eoi);
        let actual = $parser.parse(input).unwrap();
        assert_eq!(actual, $expected);
    };
    () => {};
}
