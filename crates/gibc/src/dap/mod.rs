use std::collections::HashMap;

use std::io::{stdin, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

use debugger::{ByteCodeExt as _, Debugger};
use gvm::binary::decode::decode_file;
use gvm::vm::state::ProgramState;
use responses::SetBreakpointsResponse;
use thiserror::Error;

use dap::prelude::*;
use tracing::error;
use types::Breakpoint;

pub mod debugger;

#[derive(Error, Debug)]
enum MyAdapterError {
    #[error("Unhandled command {cmd:?}")]
    UnhandledCommandError { cmd: Command },

    #[error("Missing command")]
    MissingCommandError,

    #[error("Path not found {path}")]
    PathNotFound { path: PathBuf },
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn start_dap(path: &Path) -> DynResult<()> {
    let bytes = fs::read(path).unwrap();
    let pwd = std::env::current_dir().unwrap();
    let output = BufWriter::new(std::io::stdout());
    let bytecode = Arc::new(decode_file(&mut bytes.into_iter().peekable()));
    thread::sleep(Duration::from_secs(3));
    let buf = BufReader::new(stdin());
    let mut server = Server::new(buf, output);
    let breakpoints = Arc::new(Mutex::new(HashMap::new()));
    let breakpoint_count = Arc::new(Mutex::new(0));
    let paused = Arc::new(Mutex::new(false));
    while let Some(req) = server.poll_request().unwrap() {
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
                let rsp = req.success(ResponseBody::Launch);
                let output = server.output.clone();
                let breakpoints = breakpoints.clone();
                let paused = paused.clone();
                let bytecode = bytecode.clone();
                server.respond(rsp)?;
                tokio::spawn(async move {
                    let prog = ProgramState::new(
                        &bytecode.funcs,
                        bytecode.tables.clone(),
                        bytecode.file_names.clone(),
                    );
                    let mut dbg = Debugger {
                        state: prog,
                        output,
                        breakpoints,
                        paused,
                    };
                    dbg.run_dap();
                });
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
                                format!("/{}", rel_path.display()).as_str(),
                                bp.line as usize,
                                bp.column.unwrap_or(0) as usize,
                            )
                            .unwrap();
                        let mut counter = breakpoint_count.lock().unwrap();
                        let id = *counter;
                        *counter += 1;
                        res.push(Breakpoint {
                            id: Some(id),
                            verified: true,
                            line: Some(bp.line),
                            message: Some("Breakpoint set".to_string()),
                            ..Default::default()
                        });
                        breakpoints.lock().unwrap().insert(bp_internal, id);
                    }
                }
                let rsp = req.success(ResponseBody::SetBreakpoints(SetBreakpointsResponse {
                    breakpoints: res.clone(),
                }));
                server.respond(rsp).unwrap();
            }
            cmd => {
                return Err(Box::new(MyAdapterError::UnhandledCommandError {
                    cmd: cmd.clone(),
                }))
            }
        }
    }
    Ok(())
}
