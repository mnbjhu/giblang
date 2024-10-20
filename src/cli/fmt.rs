use std::path::Path;

use chumsky::input::Input as _;
use chumsky::Parser as _;
use pretty::BoxAllocator;

use crate::{item::pretty_format, lexer::parser::lexer, parser::file_parser, util::Span};

pub fn fmt(path: &Path) {
    let text = std::fs::read_to_string(path).unwrap();
    let tokens = lexer().parse(&text).unwrap();
    let len = text.len();
    let eoi = Span::splat(len);
    let input = tokens.spanned(eoi);
    let ast = file_parser().parse(input).unwrap();
    let mut out = std::io::stdout();
    pretty_format::<_, ()>(&ast, &BoxAllocator)
        .1
        .render(70, &mut out)
        .unwrap();
}
