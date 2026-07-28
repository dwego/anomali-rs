#[cfg(test)]
mod tests {
    use anomali_rs::parser::{ParsedValue, first_significant_digit};
    use rstest::rstest;

    #[rstest]
    #[case("125", ParsedValue::Digit(1))]
    #[case("7", ParsedValue::Digit(7))]
    #[case("0.0042", ParsedValue::Digit(4))]
    #[case("-750", ParsedValue::Digit(7))]
    #[case("-0.035", ParsedValue::Digit(3))]
    #[case("+920", ParsedValue::Digit(9))]
    #[case("+0.008", ParsedValue::Digit(8))]
    #[case("00030", ParsedValue::Digit(3))]
    #[case("0000009", ParsedValue::Digit(9))]
    #[case("42.75", ParsedValue::Digit(4))]
    #[case("42,75", ParsedValue::Digit(4))]
    #[case("   125   ", ParsedValue::Digit(1))]
    #[case("\t0.0042\n", ParsedValue::Digit(4))]
    #[case("0", ParsedValue::Zero)]
    #[case("0000", ParsedValue::Zero)]
    #[case("0.000", ParsedValue::Zero)]
    #[case("0,000", ParsedValue::Zero)]
    #[case("-0.000", ParsedValue::Zero)]
    #[case("+0", ParsedValue::Zero)]
    #[case("", ParsedValue::Missing)]
    #[case("   ", ParsedValue::Missing)]
    #[case("\t\n\r", ParsedValue::Missing)]
    #[case("hello", ParsedValue::Invalid)]
    #[case("123abc", ParsedValue::Invalid)]
    #[case("abc123", ParsedValue::Invalid)]
    #[case("12-3", ParsedValue::Invalid)]
    #[case("--42", ParsedValue::Invalid)]
    #[case("++42", ParsedValue::Invalid)]
    #[case("+-42", ParsedValue::Invalid)]
    #[case("-+42", ParsedValue::Invalid)]
    #[case("+", ParsedValue::Invalid)]
    #[case("-", ParsedValue::Invalid)]
    #[case(".", ParsedValue::Invalid)]
    #[case(",", ParsedValue::Invalid)]
    #[case("..", ParsedValue::Invalid)]
    #[case(",,", ParsedValue::Invalid)]
    #[case("1.2.3", ParsedValue::Invalid)]
    #[case("1,2,3", ParsedValue::Invalid)]
    #[case("1.2,3", ParsedValue::Invalid)]
    #[case("R$ 125,00", ParsedValue::Invalid)]
    #[case("$125.00", ParsedValue::Invalid)]
    #[case("12%", ParsedValue::Invalid)]
    #[case("NaN", ParsedValue::Invalid)]
    #[case("Infinity", ParsedValue::Invalid)]
    #[case("1e3", ParsedValue::Invalid)]
    #[case("1_000", ParsedValue::Invalid)]
    fn parses_values(#[case] input: &str, #[case] expected: ParsedValue) {
        assert_eq!(
            first_significant_digit(input),
            expected,
            "unexpected result for input {input:?}"
        );
    }
}
