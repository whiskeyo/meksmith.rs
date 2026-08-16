pub use crate::smith::c::emit::to_snake_case;

pub fn fn_name(type_name: &str, suffix: &str) -> String {
    format!("{}_{suffix}", to_snake_case(type_name))
}
