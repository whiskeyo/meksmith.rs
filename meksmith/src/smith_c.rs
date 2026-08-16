use crate::analyze::analyze;
use crate::diagnostic::Diagnostics;
use crate::frontend::parser::parse;
use crate::smith::{CSmith, Smith};

pub fn generate_c_code_from_string(source: &str) -> Result<String, String> {
    let file = parse(source)
        .map_err(|diagnostics: Diagnostics| diagnostics.render_plain(source, "input.mek"))?;
    let checked =
        analyze(&file).map_err(|diagnostics| diagnostics.render_plain(source, "input.mek"))?;
    CSmith
        .generate(&checked)
        .map_err(|diagnostics| diagnostics.render_plain(source, "input.mek"))
}
