mod codec;
mod composite;
mod emit;
mod types;

use crate::analyze::CheckedFile;
use crate::diagnostic::Diagnostics;
use crate::smith::Smith;
use crate::smith::codegen::lang::Lang;

pub struct CppSmith;

impl Smith for CppSmith {
    type Output = String;

    fn name(&self) -> &'static str {
        "cpp"
    }

    fn generate(&self, file: &CheckedFile) -> Result<Self::Output, Diagnostics> {
        let mut output = String::new();
        let lang = Lang::cpp(
            &file
                .file
                .protocol
                .as_ref()
                .map(|protocol| protocol.node.to_ascii_lowercase())
                .unwrap_or_else(|| "mek".into()),
        );

        lang.preamble(&mut output);
        lang.open_scope(&mut output);

        types::emit_types(&mut output, file);
        codec::emit_codecs(&mut output, file, &lang.id);
        composite::emit_composite_codecs(&mut output, file, &lang.id);

        lang.close_scope(&mut output);
        Ok(output)
    }
}
