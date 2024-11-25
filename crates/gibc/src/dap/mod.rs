use debugger::{ByteCodeExt as _, Debugger};
use events::OutputEventBody;
use gvm::binary::decode::decode_file;
use gvm::vm::state::ProgramState;
use responses::SetBreakpointsResponse;
use std::fs;
use std::io::{stdin, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use thiserror::Error;

use dap::prelude::*;
use tracing::error;
use types::Breakpoint;

pub mod debugger;

#[derive(Error, Debug)]
enum MyAdapterError {
    #[error("Unhandled command {cmd:?}")]
    UnhandledCommandError { cmd: Command },

    #[error("Path not found {path}")]
    PathNotFound { path: PathBuf },
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn start_dap(path: &Path) -> DynResult<()> {
    let bytes = fs::read(path).unwrap();
    let pwd = std::env::current_dir().unwrap();
    let output = BufWriter::new(std::io::stdout());
    let bytecode = decode_file(&mut bytes.into_iter().peekable());
    let buf = BufReader::new(stdin());
    let mut server = Server::new(buf, output);
    let mut breakpoint_count = 0;

    let output = server.output.clone();
    let prog = ProgramState::new(
        &bytecode.funcs,
        bytecode.tables.clone(),
        bytecode.file_names.clone(),
    );
    let mut dbg = Debugger::new(prog, output);
    loop {
        dbg.poll();
        if let Some(req) = server.poll_request().unwrap() {
            match &req.command {
                Command::Initialize(_) => {
                    let rsp = req.success(ResponseBody::Initialize(types::Capabilities {
                        ..Default::default()
                    }));

                    // When you call respond, send_event etc. the message will be wrapped
                    // in a base message with a appropriate seq number, so you don't have to keep track of that yourself
                    server.respond(rsp)?;

                    server.send_event(Event::Initialized)?;
                }
                Command::Launch(_) => {
                    dbg.paused = false;
                    let text = format!("Launching {}\n", dbg.state.stack_trace());
                    server
                        .output
                        .lock()
                        .unwrap()
                        .send_event(Event::Output(OutputEventBody {
                            output: text,
                            ..Default::default()
                        }))
                        .unwrap();
                    let rsp = req.success(ResponseBody::Launch);
                    server.respond(rsp)?;
                }
                Command::SetBreakpoints(args) => {
                    // Set breakpoints
                    let Some(file) = args.source.path.as_ref() else {
                        return Err(Box::new(MyAdapterError::PathNotFound {
                            path: path.to_path_buf(),
                        }));
                    };
                    let rel_path = Path::new(file).strip_prefix(&pwd).unwrap();
                    let mut res = vec![];
                    if let Some(args) = &args.breakpoints {
                        for bp in args {
                            let bp_internal = bytecode
                                .get_breakpoint(
                                    rel_path.to_str().unwrap(),
                                    bp.line as usize,
                                    bp.column.unwrap_or(0) as usize,
                                )
                                .unwrap();
                            let id = breakpoint_count;
                            breakpoint_count += 1;
                            res.push(Breakpoint {
                                id: Some(id),
                                verified: true,
                                line: Some(bp.line),
                                message: Some("Breakpoint set".to_string()),
                                ..Default::default()
                            });
                            dbg.breakpoints.insert(bp_internal, id);
                        }
                    }
                    let rsp = req.success(ResponseBody::SetBreakpoints(SetBreakpointsResponse {
                        breakpoints: res.clone(),
                    }));

                    server
                        .output
                        .lock()
                        .unwrap()
                        .send_event(Event::Output(OutputEventBody {
                            output: format!("Setting breakpoints: {:?}\n", dbg.breakpoints),
                            ..Default::default()
                        }))
                        .unwrap();
                    server.respond(rsp).unwrap();
                }
                cmd => {
                    return Err(Box::new(MyAdapterError::UnhandledCommandError {
                        cmd: cmd.clone(),
                    }))
                }
            }
        }
    }
}
