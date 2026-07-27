
#[derive(Debug, PartialEq)]
pub enum ParsedValue {
    Digit(u8),
    Zero,
    Missing,
    Invalid,
}

pub fn first_significant_digit(value: &str) -> ParsedValue {
    if value.is_empty() {
        return ParsedValue::Missing;
    }

    let mut has_zero_digit = false;

    for i in value.chars() {
        if let Some(digit) = i.to_digit(10) {
            if digit != 0 {
                return ParsedValue::Digit(digit as u8);
            } else {
                has_zero_digit = true;
            };
        }
    }

    if has_zero_digit {
        ParsedValue::Zero
    } else {
        ParsedValue::Invalid
    }
}