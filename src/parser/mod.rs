use std::vec;

use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    lexer::{
        keyword::Keyword,
        token::{punct, Token},
    },
};
use chumsky::{
    container::Container, error::{Rich, RichPattern}, input::Input, primitive::{choice, just, none_of}, recovery::{nested_delimiters, via_parser}, IterParser, Parser
};
use salsa::Accumulator;
use tracing::info;

use crate::{
    lexer::{parser::lexer, token::newline},
    util::{Span, Spanned},
    AstParser,
};

use self::{
    common::optional_newline::optional_newline,
    top::{top_parser, Top},
};

pub mod common;
pub mod expr;
pub mod stmt;
pub mod top;

pub type File = Vec<Spanned<Top>>;

#[salsa::tracked]
pub struct Ast<'db> {
    #[return_ref]
    pub tops: Vec<Spanned<Top>>,
    pub valid: bool,
    #[return_ref]
    pub expected: Vec<Spanned<Vec<Keyword>>>,
}

#[must_use]
pub fn file_parser<'tokens, 'src: 'tokens>() -> AstParser!(File) {
    top_parser()
        .map_with(|t, e| Some((t, e.span())))
        .recover_with(via_parser(top_recovery().map(|()| None)))
        .separated_by(just(newline()))
        .collect::<Vec<_>>()
        .map(|tops: Vec<_>| tops.into_iter().flatten().collect())
        .validate(|mut v: File, _, emitter| {
            let mut existing: Vec<String> = vec![];
            v.retain(move |top| {
                if let Some(name) = top.0.get_name() {
                    if existing.contains(&name.to_string()) {
                        emitter.emit(Rich::custom(
                            top.0.name_span(),
                            format!("Duplicate top-level item '{name}'"),
                        ));
                        false
                    } else {
                        existing.push(name.to_string());
                        true
                    }
                } else {
                    true
                }
            });
            v
        })
        .padded_by(optional_newline())
}

type Unit = ();

pub struct ExpectedKeyword {
    pub kw: Keyword,
    pub span: Span,
}

#[must_use]
pub fn top_recovery<'tokens, 'src: 'tokens>() -> AstParser!(Unit) {
    let braces = nested_delimiters(
        Token::Punct('{'),
        Token::Punct('}'),
        [
            (Token::Punct('('), Token::Punct(')')),
            (Token::Punct('['), Token::Punct(']')),
        ],
        |_| (),
    );
    let parens = nested_delimiters(
        Token::Punct('('),
        Token::Punct(')'),
        [
            (Token::Punct('{'), Token::Punct('}')),
            (Token::Punct('['), Token::Punct(']')),
        ],
        |_| (),
    );
    let brackets = nested_delimiters(
        Token::Punct('['),
        Token::Punct(']'),
        [
            (Token::Punct('('), Token::Punct(')')),
            (Token::Punct('{'), Token::Punct('}')),
        ],
        |_| (),
    );
    let toks = none_of(vec![Token::Newline, punct(']'), punct(')'), punct('}')]).ignored();
    choice((braces, parens, brackets, toks))
        .repeated()
        .at_least(1)
        .then(just(Token::Newline).rewind())
        .ignored()
}

#[salsa::tracked]
pub fn parse_file<'db>(db: &'db dyn Db, file: SourceFile) -> Ast<'db> {
    info!("Parsing file: {}", file.path(db).to_str().unwrap());
    let mut valid = true;
    let mut expected = vec![];
    let text = file.text(db);
    let (tokens, errors) = lexer().parse(text).into_output_errors();
    let len = text.len();
    let mut found = vec![];
    for error in errors {
        valid = false;
        info!("Lexer error: {:?}", error);
        Diagnostic {
            message: error.reason().to_string(),
            span: *error.span(),
            level: Level::Error,
            path: file.path(db),
            file,
        }
        .accumulate(db);
    }
    if let Some(tokens) = tokens {
        let eoi = Span::splat(len);
        let input = tokens.spanned(eoi);
        let (ast, errors) = file_parser().parse(input).into_output_errors();
        for error in errors {
            valid = false;
            info!("Parser error: {:?}", error);
            let mut expected_iter = error.expected();
            let mut e = vec![];
            while let Some(RichPattern::Token(tok)) = expected_iter.next() {
                info!("Expected token: {:?}", tok);
                if let Token::Keyword(kw) = tok.clone().into_inner() {
                    info!("Expected keyword: {:?}", kw);
                    e.push(kw);
                }
            }
            Diagnostic {
                message: error.reason().to_string(),
                span: *error.span(),
                level: Level::Error,
                path: file.path(db),
                file,
            }
            .accumulate(db);
            info!("Expected all: {:?}", e);
            expected.push((e, *error.span()));
        }
        if let Some(ast) = ast {
            found = ast;
        }
    }
    Ast::new(db, found, valid, expected)
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

#[macro_export]
macro_rules! assert_parse_eq_with_errors {
    ($parser:expr, $input:expr, $expected:expr) => {
        #[allow(unused_imports)]
        use chumsky::input::Input as _;
        #[allow(unused_imports)]
        use chumsky::Parser as _;
        let tokens = $crate::lexer::parser::lexer().parse($input).unwrap();
        let eoi = $crate::util::Span::splat($input.len());
        let input = tokens.spanned(eoi);
        let (actual, errors) = $parser.parse(input).into_output_errors();
        assert!(!errors.is_empty(), "Expected errors, found none");
        assert_eq!(actual.unwrap(), $expected);
    };
    () => {};
}
