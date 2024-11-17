use crate::run::scope::Scope;
use crate::run::state::{FuncDef, ProgramState};
use crate::run::text::bc_parser;
use crate::{run::text::byte_code_lexer, util::Span};
use chumsky::input::Input as _;
use chumsky::Parser as _;
use dap::server::ServerOutput;
use events::StoppedEventBody;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{fs, path::Path};
use types::{Breakpoint, OutputEventCategory, StoppedEventReason};

use super::byte_code_text::{parse_byte_code_text, print_error};
use std::fs::File;
use std::io::{BufReader, BufWriter, Stdout, Write};

use thiserror::Error;

use dap::prelude::*;

#[derive(Error, Debug)]
enum MyAdapterError {
    #[error("Unhandled command")]
    UnhandledCommandError,

    #[error("Missing command")]
    MissingCommandError,
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct DebugAdapter<'str, 'code> {
    output: Arc<Mutex<ServerOutput<Stdout>>>,
    state: ProgramState<'code>,
    funcs: HashMap<u32, FuncDef>,
    text: &'str str,
    breakpoints: Arc<Mutex<Vec<u32>>>,
}

fn start_dap(path: &Path) -> DynResult<()> {
    let output = BufWriter::new(std::io::stdout());
    let f = File::open(path)?;
    let input = BufReader::new(f);
    let text = fs::read_to_string(path)?;
    let funcs = parse_byte_code_text(&text).unwrap();
    let mut server = Server::new(input, output);
    let break_points = Arc::new(Mutex::new(Vec::new()));
    let req = match server.poll_request()? {
        Some(req) => req,
        None => return Err(Box::new(MyAdapterError::MissingCommandError)),
    };
    match req.command {
        Command::Initialize(_) => {
            let rsp = req.success(ResponseBody::Initialize(types::Capabilities {
                ..Default::default()
            }));

            // When you call respond, send_event etc. the message will be wrapped
            // in a base message with a appropriate seq number, so you don't have to keep track of that yourself
            server.respond(rsp)?;

            server.send_event(Event::Initialized)?;
        }
        Command::Launch(args) => {
            tokio::spawn(async move {
                let mut dbg = DebugAdapter {
                    output: server.output.clone(),
                    state: ProgramState::new(),
                    text: &text,
                    breakpoints: break_points.clone(),
                    funcs,
                };
                let main = dbg.funcs.get(&0).expect("No main function");
                dbg.state.scopes.push(Scope::from_code(&main.body, 0));
                while !dbg.state.scopes.is_empty() {
                    let instr = dbg.state.next_instr();
                    println!("Executing: {instr:?} {}", dbg.state.stack_trace());
                    dbg.state.execute(instr, &dbg.funcs);
                }
            });
        }
        _ => return Err(Box::new(MyAdapterError::UnhandledCommandError)),
    }
    Ok(())
}
pub fn debug(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    let (tokens, errors) = byte_code_lexer().parse(&text).into_output_errors();
    for err in errors {
        print_error(&err, &text, path);
    }
    if let Some(tokens) = tokens {
        let parser_input = tokens.spanned(Span::splat(text.len()));
        let (funcs, errors) = bc_parser().parse(parser_input).into_output_errors();
        for err in errors {
            print_error(&err, &text, path);
        }
        if let Some(funcs) = funcs {
            let main = &funcs[&0];
            // let mut prog =
            //     ProgramState::new(&main.body, (funcs.len() as u32).checked_sub(1).unwrap());
            // prog.run(&funcs);
        }
    }
}
