use std::{collections::HashMap, fs::read_to_string};

use ariadne::Source;
use chumsky::{input::Input, Parser};

use crate::{
    cli::build::print_error,
    lexer::parser::lexer,
    parser::{file_parser, top::Top, File},
    util::Span,
};

use super::export::{Export, ExportData};

pub struct Project {
    pub exports: Export,
    pub unresolved: HashMap<String, Vec<String>>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            exports: Export::default(),
            unresolved: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file: File, path: &[String]) {
        let module = self.exports.get_or_new_path(path.iter());
        for (item, _) in file {
            if let Top::Impl(_) | Top::Use(_) = item {
                continue;
            }
            let name = item.name().to_string();
            let data = match item {
                Top::Func(func) => ExportData::Func(func),
                Top::Struct(struct_) => ExportData::Struct(struct_),
                Top::Trait(mut trait_) => {
                    let mut funcs = HashMap::new();
                    while !trait_.body.is_empty() {
                        let func = trait_.body.pop().unwrap().0;
                        funcs.insert(func.name.0.clone(), Export::new(ExportData::Func(func)));
                    }
                    ExportData::Trait(trait_, funcs)
                }
                Top::Enum(mut enum_) => {
                    let mut members = HashMap::new();
                    while !enum_.members.is_empty() {
                        let member = enum_.members.pop().unwrap().0;
                        members.insert(
                            member.name.0.clone(),
                            Export::new(ExportData::Member(member)),
                        );
                    }
                    ExportData::Enum(enum_, members)
                }
                _ => unimplemented!(),
            };
            module.insert(name, Export::new(data));
        }
    }

    pub fn parse_file(&mut self, src: String, path: &str, qualified_name: Vec<String>) {
        let source = Source::from(src.clone());
        let eoi = Span::splat(src.len());
        let (tokens, errors) = lexer().parse(&src).into_output_errors();
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
                self.insert(file, &qualified_name)
            }
        }
    }

    pub fn init_pwd() -> Project {
        let files = glob::glob("**/*.gib").unwrap();
        let mut project = Project::new();
        for file in files {
            let file = file.unwrap();
            let path = file.to_str().unwrap();
            let qualified_name = path
                .strip_suffix(".gib")
                .unwrap()
                .split('/')
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            let src = read_to_string(path).unwrap();
            project.parse_file(src, path, qualified_name);
        }
        project
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}
