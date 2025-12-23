use chumsky::prelude::*;

use crate::meklang::ast::{BuiltinType, Type};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::base::identifier;

pub(crate) const SIGNED_INTEGER_8: &str = "i8";
pub(crate) const SIGNED_INTEGER_16: &str = "i16";
pub(crate) const SIGNED_INTEGER_32: &str = "i32";
pub(crate) const SIGNED_INTEGER_64: &str = "i64";
pub(crate) const UNSIGNED_INTEGER_8: &str = "u8";
pub(crate) const UNSIGNED_INTEGER_16: &str = "u16";
pub(crate) const UNSIGNED_INTEGER_32: &str = "u32";
pub(crate) const UNSIGNED_INTEGER_64: &str = "u64";
pub(crate) const FLOAT_32: &str = "f32";
pub(crate) const FLOAT_64: &str = "f64";
pub(crate) const BIT: &str = "bit";
pub(crate) const BOOL: &str = "bool";
pub(crate) const BYTE: &str = "byte";

pub(crate) fn builtin_type<'src>() -> impl Parser<'src, &'src str, BuiltinType, ErrType<'src>> {
    choice((
        just(SIGNED_INTEGER_8).to(BuiltinType::SignedInteger8),
        just(SIGNED_INTEGER_16).to(BuiltinType::SignedInteger16),
        just(SIGNED_INTEGER_32).to(BuiltinType::SignedInteger32),
        just(SIGNED_INTEGER_64).to(BuiltinType::SignedInteger64),
        just(UNSIGNED_INTEGER_8).to(BuiltinType::UnsignedInteger8),
        just(UNSIGNED_INTEGER_16).to(BuiltinType::UnsignedInteger16),
        just(UNSIGNED_INTEGER_32).to(BuiltinType::UnsignedInteger32),
        just(UNSIGNED_INTEGER_64).to(BuiltinType::UnsignedInteger64),
        just(FLOAT_32).to(BuiltinType::Float32),
        just(FLOAT_64).to(BuiltinType::Float64),
        just(BIT).to(BuiltinType::Bit),
        just(BOOL).to(BuiltinType::Boolean),
        just(BYTE).to(BuiltinType::Byte),
    ))
}

pub(crate) fn any_type<'src>() -> impl Parser<'src, &'src str, Type, ErrType<'src>> {
    choice((
        builtin_type().map(Type::Builtin),
        identifier().map(Type::UserDefined),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use crate::meklang::ast::Identifier;

    #[rstest]
    #[case(SIGNED_INTEGER_8, BuiltinType::SignedInteger8)]
    #[case(SIGNED_INTEGER_16, BuiltinType::SignedInteger16)]
    #[case(SIGNED_INTEGER_32, BuiltinType::SignedInteger32)]
    #[case(SIGNED_INTEGER_64, BuiltinType::SignedInteger64)]
    #[case(UNSIGNED_INTEGER_8, BuiltinType::UnsignedInteger8)]
    #[case(UNSIGNED_INTEGER_16, BuiltinType::UnsignedInteger16)]
    #[case(UNSIGNED_INTEGER_32, BuiltinType::UnsignedInteger32)]
    #[case(UNSIGNED_INTEGER_64, BuiltinType::UnsignedInteger64)]
    #[case(FLOAT_32, BuiltinType::Float32)]
    #[case(FLOAT_64, BuiltinType::Float64)]
    #[case(BIT, BuiltinType::Bit)]
    #[case(BOOL, BuiltinType::Boolean)]
    #[case(BYTE, BuiltinType::Byte)]
    fn test_builtin_type(#[case] input: &str, #[case] expected: BuiltinType) {
        let result = builtin_type().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case("MyType", Type::UserDefined(Identifier::new("MyType")))]
    #[case("_type", Type::UserDefined(Identifier::new("_type")))]
    #[case(SIGNED_INTEGER_8, Type::Builtin(BuiltinType::SignedInteger8))]
    #[case(SIGNED_INTEGER_16, Type::Builtin(BuiltinType::SignedInteger16))]
    #[case(SIGNED_INTEGER_32, Type::Builtin(BuiltinType::SignedInteger32))]
    #[case(SIGNED_INTEGER_64, Type::Builtin(BuiltinType::SignedInteger64))]
    #[case(UNSIGNED_INTEGER_8, Type::Builtin(BuiltinType::UnsignedInteger8))]
    #[case(UNSIGNED_INTEGER_16, Type::Builtin(BuiltinType::UnsignedInteger16))]
    #[case(UNSIGNED_INTEGER_32, Type::Builtin(BuiltinType::UnsignedInteger32))]
    #[case(UNSIGNED_INTEGER_64, Type::Builtin(BuiltinType::UnsignedInteger64))]
    #[case(FLOAT_32, Type::Builtin(BuiltinType::Float32))]
    #[case(FLOAT_64, Type::Builtin(BuiltinType::Float64))]
    #[case(BIT, Type::Builtin(BuiltinType::Bit))]
    #[case(BOOL, Type::Builtin(BuiltinType::Boolean))]
    #[case(BYTE, Type::Builtin(BuiltinType::Byte))]
    fn test_any_type(#[case] input: &str, #[case] expected: Type) {
        let result = any_type().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
