use std::fs::read_to_string;

use ariadne::Source;
use chumsky::{input::Input, Parser};
use glob::glob;
use ptree::{print_tree, TreeBuilder};

use crate::{fs::project::Project, lexer::parser::lexer, parser::file_parser, util::Span};

use super::build::print_error;

pub fn exports() {
    let files = glob("**/*.gib").unwrap();
    let mut project = Project::new();
    for file in files {
        let file = file.unwrap();
        let path = file.to_str().unwrap();
        let qualified_name = path
            .strip_suffix(".gib")
            .unwrap()
            .split("/")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let src = read_to_string(path).unwrap();
        let eoi = Span::splat(src.len());
        let (tokens, errors) = lexer().parse(&src).into_output_errors();
        let source = Source::from(src.clone());
        for error in errors {
            print_error(error, &source, path, "Lexer Error");
        }

        if let Some(tokens) = tokens {
            let input = tokens.spanned(eoi);
            let (file, errors) = file_parser().parse(input).into_output_errors();
            for error in errors {
                print_error(error, &source, path, "Parser Error");
            }

            if let Some(file) = file {
                project.insert(file, &qualified_name)
            }
        }
    }
    let mut builder = TreeBuilder::new("/".to_string());
    project.exports.build_tree(&mut builder, "/".to_string());
    print_tree(&builder.build()).unwrap();
}
