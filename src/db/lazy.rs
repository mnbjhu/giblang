use std::{path::PathBuf, sync::Mutex, time::Duration};

use crossbeam::channel::{unbounded, Sender};
use dashmap::{mapref::entry::Entry, DashMap};
use notify_debouncer_mini::{
    new_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
    DebounceEventResult, Debouncer,
};
use salsa::{Setter, Storage};

use crate::parser::parse_file;

// ANCHOR: main
pub fn watch_test() -> ! {
    // Create the channel to receive file change events.
    let (tx, rx) = unbounded();
    let mut db = LazyInputDatabase::new(tx);

    let initial_file_path = std::env::args_os()
        .nth(1)
        .ok_or_else(|| panic!("File not found"))
        .unwrap();

    // Create the initial input using the input method so that changes to it
    // will be watched like the other files.
    let initial = db.input(initial_file_path.into());
    loop {
        // Compile the code starting at the provided input, this will read other
        // needed files using the on-demand mechanism.
        let ast = parse_file(&db, initial);
        let diagnostics = crate::parser::parse_file::accumulated::<Diagnostic>(&db, initial);
        if diagnostics.is_empty() {
            println!("Ast: {ast:?}");
        } else {
            for diagnostic in diagnostics {
                println!("{}", diagnostic.0);
            }
        }

        for log in db.logs.lock().unwrap().drain(..) {
            eprintln!("{log}");
        }

        // Wait for file change events, the output can't change unless the
        // inputs change.
        for event in rx.recv().unwrap().unwrap() {
            let path = event.path.canonicalize().unwrap();
            let file = match db.files.get(&path) {
                Some(file) => *file,
                None => continue,
            };
            // `path` has changed, so read it and update the contents to match.
            // This creates a new revision and causes the incremental algorithm
            // to kick in, just like any other update to a salsa input.
            let contents = std::fs::read_to_string(path).unwrap();
            file.set_contents(&mut db).to(contents);
        }
    }
}
// ANCHOR_END: main

// ANCHOR: db
#[salsa::input]
pub struct File {
    pub path: PathBuf,
    #[return_ref]
    pub contents: String,
}

#[salsa::db]
pub trait Db: salsa::Database {
    fn input(&self, path: PathBuf) -> File;
}

#[salsa::db]
struct LazyInputDatabase {
    storage: Storage<Self>,
    logs: Mutex<Vec<String>>,
    files: DashMap<PathBuf, File>,
    file_watcher: Mutex<Debouncer<RecommendedWatcher>>,
}

impl LazyInputDatabase {
    fn new(tx: Sender<DebounceEventResult>) -> Self {
        Self {
            storage: Storage::default(),
            logs: Mutex::default(),
            files: DashMap::new(),
            file_watcher: Mutex::new(new_debouncer(Duration::from_secs(1), tx).unwrap()),
        }
    }
}

#[salsa::db]
impl salsa::Database for LazyInputDatabase {
    fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
        // don't log boring events
        let event = event();
        if let salsa::EventKind::WillExecute { .. } = event.kind {
            self.logs.lock().unwrap().push(format!("{:?}", event));
        }
    }
}

#[salsa::db]
impl Db for LazyInputDatabase {
    fn input(&self, path: PathBuf) -> File {
        let path = path.canonicalize().unwrap();
        match self.files.entry(path.clone()) {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => *entry.get(),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                // Set up the watch before reading the contents to try to avoid
                // race conditions.
                let watcher = &mut *self.file_watcher.lock().unwrap();
                watcher
                    .watcher()
                    .watch(&path, RecursiveMode::NonRecursive)
                    .unwrap();
                let contents = std::fs::read_to_string(&path).unwrap();
                *entry.insert(File::new(self, path, contents))
            }
        }
    }
}
// ANCHOR_END: db

#[salsa::accumulator]
struct Diagnostic(String);

// #[salsa::tracked]
// fn sum<'db>(db: &'db dyn Db, input: FileData<'db>) -> u32 {
//     input.value(db)
//         + input
//             .links(db)
//             .iter()
//             .map(|&file| sum(db, file))
//             .sum::<u32>()
// }
