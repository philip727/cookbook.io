pub fn is_password_valid(s: &str) -> bool {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special_character = false;

    for c in s.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
        has_special_character |= c == '&'
            || c == '@'
            || c == '#'
            || c == '%'
            || c == '^'
            || c == '*'
            || c == '('
            || c == ')'
            || c == '!'
            || c == '?'
            || c == '<'
            || c == '>'
    }

    !has_whitespace && has_upper && has_lower && has_digit && has_special_character && s.len() >= 8
}
