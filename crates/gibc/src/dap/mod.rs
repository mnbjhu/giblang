use dashmap::DashMap;
use debugger::{ByteCodeExt as _, Debugger};
use events::{OutputEventBody, StoppedEventBody};
use gvm::binary::decode::decode_file;
use gvm::vm::state::ProgramState;
use responses::SetBreakpointsResponse;
use std::io::{stdin, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};
use thiserror::Error;

use dap::prelude::*;
use tracing::error;
use types::{Breakpoint, StoppedEventReason};

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
    let bytecode = decode_file(&mut bytes.clone().into_iter().peekable());
    let output = BufWriter::new(std::io::stdout());
    let buf = BufReader::new(stdin());
    let mut server = Server::new(buf, output);
    let mut breakpoint_count = 0;

    let output = server.output.clone();
    let breakpoints = Arc::new(DashMap::new());
    let paused = Arc::new(Mutex::new(true));
    loop {
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
                    let bytecode = decode_file(&mut bytes.clone().into_iter().peekable());
                    let breakpoints = breakpoints.clone();
                    let paused = paused.clone();
                    let o = output.clone();
                    *paused.lock().unwrap() = false;

                    let rsp = req.success(ResponseBody::Launch);
                    server.respond(rsp)?;

                    output
                        .lock()
                        .unwrap()
                        .send_event(Event::Stopped(StoppedEventBody {
                            reason: StoppedEventReason::Breakpoint,
                            description: Some("Breakpoint hit desc".to_string()),
                            thread_id: None,
                            preserve_focus_hint: Some(false),
                            text: Some("Breakpoint hit".to_string()),
                            all_threads_stopped: Some(true),
                            hit_breakpoint_ids: Some(vec![0]),
                        }))?;
                    // tokio::spawn(async move {
                    //     let prog = ProgramState::new(
                    //         &bytecode.funcs,
                    //         bytecode.tables.clone(),
                    //         bytecode.file_names.clone(),
                    //     );
                    //     let mut dbg = Debugger::new(prog, o, breakpoints.clone(), paused.clone());
                    //     loop {
                    //         thread::sleep(Duration::from_millis(10));
                    //         let _ = dbg.poll();
                    //     }
                    // });
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
                            breakpoints.insert(bp_internal, id);
                        }
                    }
                    let rsp = req.success(ResponseBody::SetBreakpoints(SetBreakpointsResponse {
                        breakpoints: res.clone(),
                    }));
                    server.respond(rsp)?;
                }
                Command::Disconnect(_) => {
                    let rsp = req.success(ResponseBody::Disconnect);
                    server.respond(rsp)?;
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
