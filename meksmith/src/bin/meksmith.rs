use std::fs;
use std::path::Path;
use std::process;

use meksmith::{
    check,
    smith::{CSmith, CppSmith, Smith},
};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        process::exit(1);
    };

    match command.as_str() {
        "check" => run_check(&mut args),
        "emit" => run_emit(&mut args),
        "help" | "-h" | "--help" => print_usage(),
        _ => {
            eprintln!("unknown command: {command}");
            print_usage();
            process::exit(1);
        }
    }
}

fn run_check(args: &mut impl Iterator<Item = String>) {
    let path = next_file_arg(args, "check");
    let (source, file_id) = read_source(&path);
    match check(&source) {
        Ok(_) => {}
        Err(diagnostics) => {
            eprint!("{}", diagnostics.render(&source, &file_id));
            process::exit(1);
        }
    }
}

fn run_emit(args: &mut impl Iterator<Item = String>) {
    let mut target = "c".to_string();
    let mut output_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" | "--target" => {
                target = args.next().unwrap_or_else(|| {
                    eprintln!("emit: missing value for {arg}");
                    process::exit(1);
                });
            }
            "-o" | "--output" => {
                output_path = Some(args.next().unwrap_or_else(|| {
                    eprintln!("emit: missing value for {arg}");
                    process::exit(1);
                }));
            }
            other if other.starts_with('-') => {
                eprintln!("emit: unknown option {other}");
                process::exit(1);
            }
            path => {
                emit_file(path, &target, output_path.as_deref());
                return;
            }
        }
    }

    eprintln!("emit: missing <file.mek>");
    process::exit(1);
}

fn emit_file(path: &str, target: &str, output_path: Option<&str>) {
    let (source, file_id) = read_source(path);
    let checked = check(&source).unwrap_or_else(|diagnostics| {
        eprint!("{}", diagnostics.render(&source, &file_id));
        process::exit(1);
    });

    let output = match target {
        "c" => CSmith.generate(&checked).unwrap_or_else(|diagnostics| {
            eprint!("{}", diagnostics.render(&source, &file_id));
            process::exit(1);
        }),
        "cpp" => CppSmith.generate(&checked).unwrap_or_else(|diagnostics| {
            eprint!("{}", diagnostics.render(&source, &file_id));
            process::exit(1);
        }),
        other => {
            eprintln!("emit: unsupported target `{other}` (supported: c, cpp)");
            process::exit(1);
        }
    };

    if let Some(output_path) = output_path {
        fs::write(output_path, output).unwrap_or_else(|err| {
            eprintln!("failed to write {output_path}: {err}");
            process::exit(1);
        });
    } else {
        print!("{output}");
    }
}

fn next_file_arg(args: &mut impl Iterator<Item = String>, command: &str) -> String {
    args.next().unwrap_or_else(|| {
        eprintln!("{command}: missing <file.mek>");
        process::exit(1);
    })
}

fn read_source(path: &str) -> (String, String) {
    let source = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("failed to read {path}: {err}");
        process::exit(1);
    });
    let file_id = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path)
        .to_string();
    (source, file_id)
}

fn print_usage() {
    eprintln!(
        "\
meksmith — meklang protocol compiler

USAGE:
    meksmith check <file.mek>
    meksmith emit [-t c|cpp] [-o out.h] <file.mek>

COMMANDS:
    check    Parse and analyze a protocol definition
    emit     Generate code from a checked protocol
"
    );
}
