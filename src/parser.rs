pub enum ParsedValue {
    Digit(u8),
    Zero,
    Missing,
    Invalid,
}

pub fn first_significant_digit(value: &str) -> ParsedValue {
    value.parse::<u8>().map_or(ParsedValue::Invalid, |digit| {
        if digit == 0 {
            ParsedValue::Zero
        } else {
            ParsedValue::Digit(digit)
        }
    })
}