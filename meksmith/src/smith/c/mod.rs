mod codec;
mod composite;
pub mod emit;
pub mod layout;
pub mod patterns;
mod types;

use crate::analyze::CheckedFile;
use crate::diagnostic::Diagnostics;
use crate::smith::Smith;

pub struct CSmith;

impl Smith for CSmith {
    type Output = String;

    fn name(&self) -> &'static str {
        "c"
    }

    fn generate(&self, file: &CheckedFile) -> Result<Self::Output, Diagnostics> {
        let mut output =
            String::from("#include <stdint.h>\n#include <stddef.h>\n#include <string.h>\n\n");

        types::emit_types(&mut output, file);

        let prefix = file
            .file
            .protocol
            .as_ref()
            .map(|protocol| protocol.node.to_ascii_lowercase())
            .unwrap_or_else(|| "mek".into());

        codec::emit_codecs(&mut output, file, &prefix);
        composite::emit_composite_codecs(&mut output, file, &prefix);

        Ok(output)
    }
}
