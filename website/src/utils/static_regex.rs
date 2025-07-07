macro_rules! static_regex {
    ($regex_name:ident, $re:literal $(,)?) => {
        static $regex_name: once_cell::sync::Lazy<regex_lite::Regex> =
            once_cell::sync::Lazy::new(|| {
                regex_lite::Regex::new($re).expect("Invalid regex pattern")
            });
    };
}

pub(crate) use static_regex;
