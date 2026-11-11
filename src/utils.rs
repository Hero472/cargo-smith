/// Converts a string to PascalCase, splitting on whitespace, underscores, and hyphens.
///
/// # Examples
/// ```
/// use cargo_smith::to_pascal_case;
/// assert_eq!(to_pascal_case("hello world"), "HelloWorld");
/// assert_eq!(to_pascal_case("user_profile"), "UserProfile");
/// ```
pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == '_' || c == '-')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_case_splits_on_separators() {
        assert_eq!(to_pascal_case("hello"), "Hello");
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
        assert_eq!(to_pascal_case("user_profile"), "UserProfile");
        assert_eq!(to_pascal_case("user-profile"), "UserProfile");
        assert_eq!(to_pascal_case(""), "");
    }
}