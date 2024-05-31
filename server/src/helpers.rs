pub fn is_alnum_whitespace(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c.is_whitespace())
}
