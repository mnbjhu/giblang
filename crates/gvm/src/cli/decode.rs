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
        let bytes = if let Some(input) = &self.input {
            fs::read(input).unwrap()
        } else {
            let mut bytes = vec![];
            stdin().read_to_end(&mut bytes).unwrap();
            bytes
        };
        let bytecode = decode_file(&mut bytes.into_iter().peekable());

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
