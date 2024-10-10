use std::vec;

use crate::db::{
    err::{Diagnostic, Level},
    input::{Db, SourceFile},
};
use chumsky::{error::Rich, input::Input, primitive::just, IterParser, Parser};
use salsa::Accumulator;
use top::impl_::Impl;
use tracing::info;

use crate::{
    // cli::build::print_error,
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
pub struct FileData<'db> {
    #[return_ref]
    pub tops: Vec<TopData<'db>>,

    #[return_ref]
    pub impls: Vec<ImplData<'db>>,
}

#[salsa::tracked]
pub struct TopData<'db> {
    #[id]
    #[interned]
    pub name: String,
    #[return_ref]
    pub data: Top,
}

#[salsa::tracked]
pub struct ImplData<'db> {
    #[return_ref]
    pub data: Impl,
}

#[must_use]
pub fn file_parser<'tokens, 'src: 'tokens>() -> AstParser!(File) {
    top_parser()
        .map_with(|t, e| (t, e.span()))
        .separated_by(just(newline()))
        .collect()
        .padded_by(optional_newline())
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
}

#[salsa::tracked]
pub fn parse_file<'db>(db: &'db dyn Db, file: SourceFile) -> FileData<'db> {
    info!("Parsing file: {:?}", file.path(db));
    let text = file.text(db);
    let (tokens, errors) = lexer().parse(text).into_output_errors();
    let len = text.len();
    let mut found = vec![];
    for error in errors {
        Diagnostic {
            message: error.reason().to_string(),
            span: *error.span(),
            level: Level::Error,
            path: file.path(db),
        }
        .accumulate(db);
    }
    if let Some(tokens) = tokens {
        let eoi = Span::splat(len);
        let input = tokens.spanned(eoi);
        let (ast, errors) = file_parser().parse(input).into_output_errors();
        for error in errors {
            Diagnostic {
                message: error.reason().to_string(),
                span: *error.span(),
                level: Level::Error,
                path: file.path(db),
            }
            .accumulate(db);
        }
        if let Some(ast) = ast {
            found = ast;
        }
    }
    FileData::new(
        db,
        found
            .iter()
            .filter_map(|(top, _)| {
                top.get_name()
                    .map(|name| TopData::new(db, name.to_string(), top.clone()))
            })
            .collect(),
        vec![],
    )
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
