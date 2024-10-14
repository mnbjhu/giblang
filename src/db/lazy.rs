// ANCHOR: main
// pub fn watch_test() -> ! {
//     // Create the channel to receive file change events.
//     let mut db = LazyInputDatabase::new();
//
//     let initial_file_path = std::env::args_os()
//         .nth(1)
//         .ok_or_else(|| panic!("File not found"))
//         .unwrap();
//
//     // Create the initial input using the input method so that changes to it
//     // will be watched like the other files.
//     let initial = db.input(initial_file_path.into());
//     loop {
//         // Compile the code starting at the provided input, this will read other
//         // needed files using the on-demand mechanism.
//         let ast = parse_file(&db, initial);
//         let diagnostics = crate::parser::parse_file::accumulated::<Diagnostic>(&db, initial);
//         if diagnostics.is_empty() {
//             println!("Ast: {ast:?}");
//         } else {
//             for diagnostic in diagnostics {
//                 println!("{}", diagnostic.0);
//             }
//         }
//
//         for log in db.logs.lock().unwrap().drain(..) {
//             eprintln!("{log}");
//         }
//
//         // Wait for file change events, the output can't change unless the
//         // inputs change.
//         for event in rx.recv().unwrap().unwrap() {
//             let path = event.path.canonicalize().unwrap();
//             let file = match db.files.get(&path) {
//                 Some(file) => *file,
//                 None => continue,
//             };
//             // `path` has changed, so read it and update the contents to match.
//             // This creates a new revision and causes the incremental algorithm
//             // to kick in, just like any other update to a salsa input.
//             let contents = std::fs::read_to_string(path).unwrap();
//             file.set_contents(&mut db).to(contents);
//         }
//     }
// }
// ANCHOR_END: main

// ANCHOR: db
// #[salsa::input]
// pub struct File {
//     pub path: PathBuf,
//     #[return_ref]
//     pub contents: String,
// }
//
// #[salsa::db]
// pub trait Db: salsa::Database {
//     fn input(&self, path: PathBuf) -> File;
// }
//
// #[salsa::db]
// pub struct LazyInputDatabase {
//     storage: Storage<Self>,
//     logs: Mutex<Vec<String>>,
//     files: DashMap<PathBuf, File>,
//     module: Option<Vfs>,
// }
//
// impl Default for LazyInputDatabase {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// impl LazyInputDatabase {
//     #[must_use]
//     pub fn new() -> Self {
//         Self {
//             storage: Storage::default(),
//             logs: Mutex::default(),
//             files: DashMap::new(),
//             module: None,
//         }
//     }
// }
//
// #[salsa::db]
// impl salsa::Database for LazyInputDatabase {
//     fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
//         // don't log boring events
//         let event = event();
//         if let salsa::EventKind::WillExecute { .. } = event.kind {
//             self.logs.lock().unwrap().push(format!("{event:?}"));
//         }
//     }
// }
//
// #[salsa::db]
// impl Db for LazyInputDatabase {
//     fn input(&self, path: PathBuf) -> File {
//         let path = path.canonicalize().unwrap();
//         match self.files.entry(path.clone()) {
//             // If the file already exists in our cache then just return it.
//             Entry::Occupied(entry) => *entry.get(),
//             // If we haven't read this file yet set up the watch, read the
//             // contents, store it in the cache, and return it.
//             Entry::Vacant(entry) => {
//                 let contents = std::fs::read_to_string(&path).unwrap();
//                 let file = File::new(self, path, contents);
//                 *entry.insert(file)
//             }
//         }
//     }
// }
