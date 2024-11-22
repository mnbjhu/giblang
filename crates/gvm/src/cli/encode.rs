use std::{
    fs::{self, OpenOptions},
    io::{stdin, Read as _, Write as _},
    path::PathBuf,
};

use crate::{
    binary::encode::encode_program,
    text::decode::parser::{parse_text_file, ParseError},
};
use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::Args;

// Convert from the text format to the binary format
#[derive(Args)]
pub struct Encode {
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

impl Encode {
    pub fn run(&self) {
        let input = if let Some(input) = &self.input {
            fs::read_to_string(input).unwrap()
        } else {
            let mut text = String::new();
            stdin().read_to_string(&mut text).unwrap();
            text
        };
        match &parse_text_file(&input) {
            Ok(bytecode) => {
                let bytes = encode_program(bytecode);
                if let Some(output) = &self.output {
                    let mut file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(output)
                        .unwrap();
                    file.write_all(&bytes).unwrap();
                } else {
                    std::io::stdout().write_all(&bytes).unwrap();
                }
            }
            Err(err) => {
                let range = if let ParseError::LexError { range }
                | ParseError::UnexpectedToken { range, .. }
                | ParseError::ParseIntError { range, .. } = err
                {
                    range
                } else {
                    let eof = input.len();
                    &(eof..eof)
                };
                let source = Source::from(&input);
                let input_name = self
                    .input
                    .as_deref()
                    .map_or("stdin", |p| p.to_str().unwrap())
                    .to_string();
                let mut report = Report::build(ReportKind::Error, (&input_name, range.clone()));
                report.set_message("An error occurred while parsing the input text");
                report.add_label(
                    Label::new((&input_name, range.clone()))
                        .with_message(err.to_string())
                        .with_color(Color::Red),
                );
                if let ParseError::UnexpectedToken { expected, .. } = err {
                    report.set_help(format!("Expected {}", expected));
                }
                report
                    .finish()
                    .eprint((&input_name, source))
                    .expect("Failed to print report");
            }
        };
    }
}
