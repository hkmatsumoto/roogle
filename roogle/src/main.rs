use core::panic;
use std::path::{Path, PathBuf};

use nom::error::ErrorKind;
use roogle_engine::{exec::QueryExecutor, parse::parse_query};
use rustdoc_types::Crate;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(short, long, parse(from_os_str))]
    index: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    query: Option<PathBuf>,
}

fn read_json(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(path.as_ref()).expect("failed in reading file")
}

fn main() {
    let cfg = Config::from_args();
    let krate = serde_json::from_str::<Crate>(&read_json(cfg.index))
        .expect("msfailed in deserializing crate");
    dbg!(krate.index.len());

    let qe = QueryExecutor::new(krate);

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let query = parse_query::<(&str, ErrorKind)>(&line)
                    .expect("failed in parsing query")
                    .1;
                let items = qe.exec(query);
                dbg!(items.iter().take(3).collect::<Vec<_>>());
            }
            Err(ReadlineError::Interrupted) => break,
            _ => panic!("exitted repl"),
        }
    }
}