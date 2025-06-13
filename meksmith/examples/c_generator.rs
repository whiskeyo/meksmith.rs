use meksmith::smith::c_smith::generate_c_code_from_string;

fn main() {
    let input = r#"
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

    let c_code = generate_c_code_from_string(input).expect("Failed to generate C code");
    println!("{}", c_code);
}
