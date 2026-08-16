pub mod analyze;
pub mod diagnostic;
pub mod frontend;
pub mod smith;
pub mod smith_c;
pub mod smith_cpp;

pub use analyze::{CheckedFile, analyze};
pub use diagnostic::{Diagnostic, DiagnosticCode, Diagnostics, Emitter, Severity};
pub use frontend::{File, parse, parse_source};
pub use smith::{CSmith, CppSmith, Smith};

pub fn check(source: &str) -> Result<CheckedFile, Diagnostics> {
    let file = frontend::parser::parse(source)?;
    analyze(&file)
}

pub fn check_path(path: impl AsRef<std::path::Path>) -> Result<CheckedFile, Diagnostics> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path).map_err(|err| {
        Diagnostics::from(Diagnostic::error(
            DiagnosticCode::E0001Unexpected,
            frontend::span::SimpleSpan::new(0, 0),
            format!("failed to read {}: {err}", path.display()),
        ))
    })?;
    check(&source)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::smith::{CSmith, Smith};

    #[test]
    fn check_ecpri_example() {
        let source = include_str!("../examples/ecpri.mek");
        let checked = check(source).expect("ecpri.mek should analyze cleanly");
        assert!(!checked.emit_order.is_empty());
        assert!(checked.symbols.get("Message").is_some());
    }

    #[test]
    fn check_generics_sketch_example() {
        let source = include_str!("../examples/generics-sketch.mek");
        let checked = check(source).expect("generics-sketch.mek should analyze cleanly");
        assert_eq!(checked.file.items.len(), 6);
    }

    #[test]
    fn emit_c_types_for_ecpri() {
        let source = include_str!("../examples/ecpri.mek");
        let checked = check(source).expect("ecpri.mek should analyze cleanly");
        let output = CSmith
            .generate(&checked)
            .expect("ecpri.mek should emit C types");
        assert!(output.contains("typedef struct"));
        assert!(output.contains("Message"));
        assert!(output.contains("Payload"));
    }

    #[test]
    fn smith_c_wrapper_roundtrip() {
        let source = include_str!("../examples/ecpri.mek");
        let output = smith_c::generate_c_code_from_string(source).expect("wrapper should work");
        assert!(output.contains("typedef enum"));
    }

    #[test]
    fn c_enum_expands_ranged_variants() {
        let source = r#"protocol Demo;

enumerated(msb0, 4 bits) MyEnum {
    x = 1,
    y = 2..4,
    z = 5,
}

structure(msb0) MyStruct {
    tag: MyEnum,
}"#;
        let output = smith_c::generate_c_code_from_string(source).expect("range enum should emit");
        assert!(output.contains("MyEnum_x = 1"));
        assert!(output.contains("MyEnum_y_2 = 2"));
        assert!(output.contains("MyEnum_y_3 = 3"));
        assert!(output.contains("MyEnum_y_4 = 4"));
        assert!(output.contains("MyEnum_z = 5"));
        assert!(!output.contains("MyEnum_y = 2"));
    }
}
