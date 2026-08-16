use crate::frontend::ast::EnumValue;
use crate::frontend::token::Literal;

/// One emitted enum constant: `(variant_name, numeric_value)`.
///
/// Scalar variants keep the meklang name (`x` → `x`). Ranged variants expand to
/// `name_start`, `name_start+1`, … `name_end` (e.g. `y = 2..4` → `y_2`, `y_3`, `y_4`).
pub fn expand_enum_variant(variant_name: &str, value: &EnumValue) -> Vec<(String, u64)> {
    match value {
        EnumValue::Integer(literal) => literal
            .as_u64()
            .map(|value| vec![(variant_name.to_string(), value)])
            .unwrap_or_default(),
        EnumValue::Range { start, end } => {
            let Some(start) = start.as_u64() else {
                return Vec::new();
            };
            let Some(end) = end.as_u64() else {
                return Vec::new();
            };
            if end < start {
                return Vec::new();
            }
            (start..=end)
                .map(|value| (format!("{variant_name}_{value}"), value))
                .collect()
        }
    }
}

/// C/C++ enum constant names with a type prefix (`MyEnum_y_2`, …).
pub fn expanded_prefixed_enum_cases(
    enum_name: &str,
    variant_name: &str,
    value: &EnumValue,
) -> Vec<String> {
    expand_enum_variant(variant_name, value)
        .into_iter()
        .map(|(name, _)| format!("{enum_name}_{name}"))
        .collect()
}

trait LiteralAsU64 {
    fn as_u64(&self) -> Option<u64>;
}

impl LiteralAsU64 for Literal {
    fn as_u64(&self) -> Option<u64> {
        match self {
            Literal::Integer { value, .. } if *value >= 0 => Some(*value as u64),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::token::Literal;

    fn int(value: u64) -> Literal {
        Literal::Integer {
            value: value as i64,
            radix: 10,
        }
    }

    #[test]
    fn scalar_variant_keeps_name() {
        let entries = expand_enum_variant("x", &EnumValue::Integer(int(1)));
        assert_eq!(entries, vec![("x".into(), 1)]);
    }

    #[test]
    fn range_variant_expands_each_value() {
        let entries = expand_enum_variant(
            "y",
            &EnumValue::Range {
                start: int(2),
                end: int(4),
            },
        );
        assert_eq!(
            entries,
            vec![("y_2".into(), 2), ("y_3".into(), 3), ("y_4".into(), 4)]
        );
    }
}
