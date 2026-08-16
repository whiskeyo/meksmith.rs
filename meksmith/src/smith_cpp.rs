use crate::analyze::analyze;
use crate::diagnostic::Diagnostics;
use crate::frontend::parser::parse;
use crate::smith::{CppSmith, Smith};

pub fn generate_cpp_code_from_string(source: &str) -> Result<String, String> {
    let file = parse(source)
        .map_err(|diagnostics: Diagnostics| diagnostics.render_plain(source, "input.mek"))?;
    let checked =
        analyze(&file).map_err(|diagnostics| diagnostics.render_plain(source, "input.mek"))?;
    CppSmith
        .generate(&checked)
        .map_err(|diagnostics| diagnostics.render_plain(source, "input.mek"))
}
