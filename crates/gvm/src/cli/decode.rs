use std::{
    fs::{self, OpenOptions},
    io::{stdin, Read, Write},
    path::PathBuf,
};

use clap::Args;

use crate::binary::decode::decode_file;

// Convert from the binary format to the text format
#[derive(Args)]
pub struct Decode {
    /// The input binary file to decode (if not provided, reads from stdin)
    pub input: Option<PathBuf>,

    /// The output text file to write to (if not provided, writes to stdout)
    pub output: Option<PathBuf>,
}

impl Decode {
    pub fn run(&self) {
        let input = if let Some(input) = &self.input {
            fs::read_to_string(input).unwrap()
        } else {
            let mut text = String::new();
            stdin().read_to_string(&mut text).unwrap();
            text
        };
        let bytecode = decode_file(&mut input.bytes().peekable());

        if let Some(output) = &self.output {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(output)
                .unwrap();
            writeln!(file, "{bytecode}").unwrap();
        }
        {
            println!("{bytecode}")
        }
    }
}
