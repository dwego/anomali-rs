#[cfg(test)]
mod tests {
    use anomali_rs::benford::BenfordError;
    use anomali_rs::benford::analyzer::{BenfordAnalyzer, expected_frequency};
    use anomali_rs::parser::ParsedValue;
    use rstest::rstest;

    const EPSILON: f64 = 1e-6;

    #[allow(clippy::approx_constant)]
    #[rstest]
    #[case(1, 0.301030)]
    #[case(2, 0.176091)]
    #[case(3, 0.124939)]
    #[case(4, 0.096910)]
    #[case(5, 0.079181)]
    #[case(6, 0.066947)]
    #[case(7, 0.057992)]
    #[case(8, 0.051153)]
    #[case(9, 0.045757)]
    fn calculates_expected_frequency(#[case] digit: u8, #[case] expected: f64) {
        let actual = expected_frequency(digit);

        assert!((actual - expected).abs() < EPSILON);
    }

    #[test]
    fn starts_with_empty_counters() {
        let analyzer = BenfordAnalyzer::new();

        assert_eq!(analyzer.counts, [0; 9]);
        assert_eq!(analyzer.analyzed, 0);
        assert_eq!(analyzer.zeros, 0);
        assert_eq!(analyzer.missing, 0);
        assert_eq!(analyzer.invalid, 0);
    }

    #[test]
    fn counts_valid_digits() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(4));
        analyzer.push(ParsedValue::Digit(9));

        assert_eq!(analyzer.counts, [2, 0, 0, 1, 0, 0, 0, 0, 1]);
        assert_eq!(analyzer.analyzed, 4);
    }

    #[test]
    fn counts_zero_missing_and_invalid_values() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Zero);
        analyzer.push(ParsedValue::Zero);
        analyzer.push(ParsedValue::Missing);
        analyzer.push(ParsedValue::Invalid);

        assert_eq!(analyzer.zeros, 2);
        assert_eq!(analyzer.missing, 1);
        assert_eq!(analyzer.invalid, 1);
        assert_eq!(analyzer.analyzed, 0);
    }

    #[test]
    fn treats_out_of_range_digits_as_invalid() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(0));
        analyzer.push(ParsedValue::Digit(10));

        assert_eq!(analyzer.invalid, 2);
        assert_eq!(analyzer.analyzed, 0);
        assert_eq!(analyzer.counts, [0; 9]);
    }

    #[test]
    fn returns_error_when_no_values_can_be_analyzed() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Zero);
        analyzer.push(ParsedValue::Missing);
        analyzer.push(ParsedValue::Invalid);

        let result = analyzer.finish();

        assert_eq!(result, Err(BenfordError::NoAnalyzableValues));
    }

    #[test]
    fn creates_a_report_with_correct_counts() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(2));
        analyzer.push(ParsedValue::Digit(4));
        analyzer.push(ParsedValue::Zero);
        analyzer.push(ParsedValue::Missing);
        analyzer.push(ParsedValue::Invalid);

        let report = analyzer.finish().expect("report should be created");

        assert_eq!(report.analyzed, 4);
        assert_eq!(report.zeros, 1);
        assert_eq!(report.missing, 1);
        assert_eq!(report.invalid, 1);

        assert_eq!(report.digits[0].digit, 1);
        assert_eq!(report.digits[0].count, 2);

        assert_eq!(report.digits[1].digit, 2);
        assert_eq!(report.digits[1].count, 1);

        assert_eq!(report.digits[3].digit, 4);
        assert_eq!(report.digits[3].count, 1);
    }

    #[test]
    fn calculates_observed_frequencies() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(2));
        analyzer.push(ParsedValue::Digit(4));

        let report = analyzer.finish().expect("report should be created");

        assert!((report.digits[0].observed_frequency - 0.5).abs() < EPSILON);
        assert!((report.digits[1].observed_frequency - 0.25).abs() < EPSILON);
        assert!((report.digits[2].observed_frequency - 0.0).abs() < EPSILON);
        assert!((report.digits[3].observed_frequency - 0.25).abs() < EPSILON);
    }

    #[test]
    fn calculates_difference_between_observed_and_expected() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(2));
        analyzer.push(ParsedValue::Digit(4));

        let report = analyzer.finish().expect("report should be created");

        let expected_difference = 0.5 - expected_frequency(1);

        assert!((report.digits[0].difference - expected_difference).abs() < EPSILON);
    }

    #[test]
    fn calculates_mean_absolute_deviation() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Digit(2));
        analyzer.push(ParsedValue::Digit(4));

        let report = analyzer.finish().expect("report should be created");

        let expected_mad = report
            .digits
            .iter()
            .map(|result| result.difference.abs())
            .sum::<f64>()
            / 9.0;

        assert!((report.mad - expected_mad).abs() < EPSILON);
    }

    #[test]
    fn report_calculates_skipped_values() {
        let mut analyzer = BenfordAnalyzer::new();

        analyzer.push(ParsedValue::Digit(1));
        analyzer.push(ParsedValue::Zero);
        analyzer.push(ParsedValue::Missing);
        analyzer.push(ParsedValue::Invalid);

        let report = analyzer.finish().expect("report should be created");

        assert_eq!(report.skipped(), 3);
        assert_eq!(report.total_received(), 4);
    }

    #[test]
    fn observed_frequencies_sum_to_one() {
        let mut analyzer = BenfordAnalyzer::new();

        for digit in 1..=9 {
            analyzer.push(ParsedValue::Digit(digit));
        }

        let report = analyzer.finish().expect("report should be created");

        let total = report
            .digits
            .iter()
            .map(|result| result.observed_frequency)
            .sum::<f64>();

        assert!((total - 1.0).abs() < EPSILON);
    }

    #[test]
    fn expected_frequencies_sum_to_one() {
        let total = (1..=9).map(expected_frequency).sum::<f64>();

        assert!((total - 1.0).abs() < EPSILON);
    }
}
