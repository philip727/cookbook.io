pub fn is_alnum_whitespace_and_ex_chars(s: &str) -> bool {
    if s.is_empty() {
        return false;
    };

    s.chars().all(|c| {
        c.is_alphanumeric()
            || c.is_ascii_whitespace()
            || c.is_ascii_punctuation()
            || c.is_ascii_alphabetic()
    })
}

pub fn is_alnum_whitespace(s: &str) -> bool {
    if s.is_empty() {
        return false;
    };

    s.chars().all(|c| c.is_alphanumeric() || c.is_whitespace())
}
