use super::build::print_error;
use crate::{lexer::parser::lexer, parser::file_parser, util::Span};
use ariadne::{Color, Fmt, Source};
use chumsky::{input::Input, Parser};
use std::fs;

pub fn parse(path: &str) {
    let src = fs::read_to_string(path).unwrap();
    let eoi = Span::splat(src.len());
    let (tokens, errors) = lexer().parse(&src).into_output_errors();
    let source = Source::from(src.clone());
    let mut success = true;
    for error in errors {
        print_error(error, source.clone(), path, "Lexer");
        success = false
    }

    if let Some(tokens) = tokens {
        let input = tokens.spanned(eoi);
        let (_, errors) = file_parser().parse(input).into_output_errors();
        for error in errors {
            print_error(error, source.clone(), path, "Parser");
            success = false
        }

        if success {
            println!("{}", "[Success]".fg(Color::Green));
        } else {
            println!("{}", "[Failed]".fg(Color::Red));
        }
    }
}
