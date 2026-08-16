pub mod c;
pub mod codegen;
pub mod cpp;
pub mod wire_ir;

use crate::analyze::CheckedFile;
use crate::diagnostic::Diagnostics;

pub trait Smith {
    type Output;

    fn name(&self) -> &'static str;

    fn generate(&self, file: &CheckedFile) -> Result<Self::Output, Diagnostics>;
}

pub use c::CSmith;
pub use cpp::CppSmith;
