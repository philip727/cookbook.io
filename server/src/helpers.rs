pub fn is_alnum_whitespace_and_ex_chars(s: &str) -> bool {
    if s.is_empty() {
        return false;
    };

    s.chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '\'' || c == ',' || c == '.')
}

pub fn is_alnum_whitespace(s: &str) -> bool {
    if s.is_empty() {
        return false;
    };

    s.chars().all(|c| c.is_alphanumeric() || c.is_whitespace())
}
