extern crate built;
extern crate colored;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io;
use std::io::Write;
use std::process;

use colored::*;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    env_logger::init();
    run_repl();
}

fn run_repl() -> Result<(), io::Error> {
    println!("TinyDB REPL: Press ctrl+c to exit");
    println!("Version {} {} {}", built_info::PKG_VERSION, built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)), built::util::strptime(built_info::BUILT_TIME_UTC).rfc3339());

    let mut buf = String::new();
    loop {
        print!("{}", "TinyDB >> ".green());
        io::stdout().flush()?;
        io::stdin().read_line(&mut buf)?;
        trace!("REPL input: {}", buf.trim());

        if buf.trim() == ".quit" {
            println!("Bye!");
            process::exit(0)
        } else {
            println!("{} `{}`", "Unrecognized command".red(), buf.trim());
        }

        buf.clear()
    }
}
