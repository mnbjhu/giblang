use ariadne::Source;
use chumsky::{error::Rich, input::Input, primitive::just, IterParser, Parser};
use ptree::TreeBuilder;

use crate::{
    cli::build::print_error,
    fs::tree_node::FileState,
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

pub fn build_tree(FileState { ast, .. }: &FileState, name: &str, builder: &mut TreeBuilder) {
    builder.begin_child(name.to_string());
    for (item, _) in ast {
        if let Some(name) = item.get_name() {
            let mut text = name.to_string();
            if let Some(impls) = item.impls() {
                let impl_names = impls
                    .iter()
                    .filter_map(|impl_| {
                        if let Some(trait_) = &impl_.impl_.trait_ {
                            Some(trait_)
                        } else {
                            None
                        }
                    })
                    .map(|trait_| trait_.0.name.last().unwrap().0.clone())
                    .collect::<Vec<String>>();

                let impls = impl_names.join(", ");
                text = format!("{name} ({impls})")
            }
            builder.add_empty_child(text);
        }
    }
    builder.end_child();
}

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
                            format!("Duplicate top-level item '{}'", name),
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

pub fn parse_file(txt: &str, filename: &str, src: &Source) -> File {
    let (tokens, errors) = lexer().parse(txt).into_output_errors();
    let len = txt.len();
    for error in errors {
        print_error(error, src.clone(), filename, "Lex");
    }
    if let Some(tokens) = tokens {
        let eoi = Span::splat(len);
        let input = tokens.spanned(eoi);
        let (ast, errors) = file_parser().parse(input).into_output_errors();
        for error in errors {
            print_error(error, src.clone(), filename, "Parse");
        }
        if let Some(ast) = ast {
            return ast;
        }
    }
    vec![]
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
