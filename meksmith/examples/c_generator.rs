static EXAMPLE_INPUT: &str = r#"
using FilePath = byte[100];

enum LogLevel {
    debug = 0;
    info = 1;
    fatal = 2;
};

enum LogFormat {
    simple = 0;
    rich = 1;
};

struct RichLog {
    file: FilePath;
    line: uint16;
    column: uint8;
    message: byte[];
};

union LogMessage {
    0 => simpleLog: byte[];
    1 => richLog: RichLog;
};

struct Log {
    logLevel: LogLevel;
    logFormat: LogFormat;
    [discriminated_by=logFormat]
    logMessage: LogMessage;
};
"#;

fn main() {
    let input = if let Some(path) = std::env::args().nth(1) {
        match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("Failed to read file '{path}', using example input.");
                EXAMPLE_INPUT.to_string()
            }
        }
    } else {
        EXAMPLE_INPUT.to_string()
    };

    match meksmith::smith_c::generate_c_code_from_string(&input) {
        Ok(code) => println!("{code}"),
        Err(e) => eprintln!("Error generating C code: {e}"),
    }
}
