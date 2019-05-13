//! A [Whitespace][1] interpreter written in Rust.
//!
//! The source code when run is compiled into bytecode and run on a simple
//! VM like many interpreted languages.
//!
//! The translation is mostly verbatim with the exception of labels, which
//! are translated into their equivalent jump targets at compile time.
//!
//! Furthermore, the VM is designed to give a full stack traceback on error,
//! unlike the reference implementation.
//!
//! ## Limitations
//! There are several limitations that prevent this implementation from
//! being fully compliant with the reference implementation in Haskell.
//! The most notable is all integers are restricted in size to standard 64-bit
//! signed integers instead of arbitrary precision integers. This was done
//! mostly for simplicity's sake.
//!
//! The stack traceback probably has some runtime cost associated with managing
//! the virtual call stack.
//!
//! [1]: https://en.wikipedia.org/wiki/Whitespace_(programming_language)

use crate::parser::Parser;
use crate::vm::Vm;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

mod parser;
mod program;
mod token;
mod vm;

/// Runs a given Whitespace program where the source code is
/// represented by `source`
pub fn run_source(source: &str) {
    let parser = Parser::new(source);
    let program = match parser.parse() {
        Ok(x) => x,
        Err(error) => {
            error.print_error();
            return;
        }
    };

    let vm = Vm::new(&program);
    if let Err(traceback) = vm.run() {
        traceback.dump();
    }
}

/// Runs a given Whitespace program where the source code is
/// stored in the file given by `filename`
pub fn run_file<P: AsRef<Path>>(filename: P) {
    let err_msg = filename.as_ref().display().to_string();
    let mut file = match OpenOptions::new().read(true).open(filename) {
        Ok(x) => x,
        Err(_) => {
            println!("Could not open '{}'", err_msg);
            return;
        }
    };

    let mut source = String::new();
    if file.read_to_string(&mut source).is_err() {
        println!("Error reading file");
    }

    run_source(&source);
}
