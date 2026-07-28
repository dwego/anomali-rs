#[derive(Debug, PartialEq, Eq)]
pub enum ParsedValue {
    Digit(u8),
    Zero,
    Missing,
    Invalid,
}

pub fn first_significant_digit(raw: &str) -> ParsedValue {
    let value = raw.trim();

    if value.is_empty() {
        return ParsedValue::Missing;
    }

    let unsigned = value
        .strip_prefix('+')
        .or_else(|| value.strip_prefix('-'))
        .unwrap_or(value);

    if unsigned.is_empty() {
        return ParsedValue::Invalid;
    }

    let mut has_digit = false;
    let mut has_separator = false;

    for character in unsigned.chars() {
        match character {
            '0'..='9' => {
                has_digit = true;
            }

            '.' | ',' if !has_separator => {
                has_separator = true;
            }

            _ => {
                return ParsedValue::Invalid;
            }
        }
    }

    if !has_digit {
        return ParsedValue::Invalid;
    }

    unsigned
        .chars()
        .find(|character| matches!(character, '1'..='9'))
        .map(|character| ParsedValue::Digit((character as u8) - b'0'))
        .unwrap_or(ParsedValue::Zero)
}
