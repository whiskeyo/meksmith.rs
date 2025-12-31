static EXAMPLE_INPUT: &str = r#"
bitenum(1) Switch {
    off = 0,
    on = 1
}

bitenum(2) Brightness {
    quarter = 0,
    half = 1,
    three_quarters = 2,
    full = 3
}

bitstruct Color(msb0) {
    bit +8 red: u8,
    bit +8 green: u8,
    bit +8 blue: u8,
    bit +8 alpha: u8
}

bitstruct Light(msb0) {
    bit derived switch_status: Switch,
    bit derived brightness: Brightness,
    bit derived color: Color
}
"#;

fn main() {
    let input = if let Some(path) = std::env::args().nth(1) {
        meksmith::meklang2::InputType::File { file_path: path }
    } else {
        meksmith::meklang2::InputType::String {
            protocol: EXAMPLE_INPUT.to_string(),
        }
    };

    match meksmith::meklang2::parse_module(input) {
        Ok(module_ast) => println!("{:#?}", module_ast),
        Err(e) => eprintln!("{e}"),
    }
}
