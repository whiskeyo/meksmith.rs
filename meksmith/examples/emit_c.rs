//! Emit C type definitions from a `.mek` file.
//!
//! ```sh
//! cargo run -p meksmith --example emit_c -- examples/ecpri.mek
//! ```

use std::{env, fs, process};

use meksmith::{
    check,
    smith::{CSmith, Smith},
};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: emit_c <file.mek>");
        process::exit(1);
    });

    let source = fs::read_to_string(&path).unwrap_or_else(|err| {
        eprintln!("failed to read {path}: {err}");
        process::exit(1);
    });

    let checked = check(&source).unwrap_or_else(|diagnostics| {
        eprint!("{}", diagnostics.render(&source, &path));
        process::exit(1);
    });

    let output = CSmith.generate(&checked).unwrap_or_else(|diagnostics| {
        eprint!("{}", diagnostics.render(&source, &path));
        process::exit(1);
    });

    print!("{output}");
}
