use crate::benford::benford_error::BenfordError;
use crate::parser::ParsedValue;

#[derive(Debug, Default)]
pub struct BenfordAnalyzer {
    pub counts: [u64; 9],
    pub analyzed: u64,
    pub zeros: u64,
    pub missing: u64,
    pub invalid: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DigitResult {
    pub digit: u8,
    pub count: u64,
    pub observed_frequency: f64,
    pub expected_frequency: f64,
    pub difference: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenfordReport {
    pub digits: [DigitResult; 9],
    pub analyzed: u64,
    pub zeros: u64,
    pub missing: u64,
    pub invalid: u64,
    pub mad: f64,
}

impl BenfordAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: ParsedValue) {
        match value {
            ParsedValue::Digit(digit @ 1..=9) => {
                self.counts[usize::from(digit - 1)] += 1;
                self.analyzed += 1;
            }
            ParsedValue::Digit(_) | ParsedValue::Invalid => {
                self.invalid += 1;
            }
            ParsedValue::Zero => {
                self.zeros += 1;
            }
            ParsedValue::Missing => {
                self.missing += 1;
            }
        }
    }

    pub fn finish(self) -> Result<BenfordReport, BenfordError> {
        if self.analyzed == 0 {
            return Err(BenfordError::NoAnalyzableValues);
        }

        let digits = std::array::from_fn(|index| {
            let digit = index as u8 + 1;
            let count = self.counts[index];
            let observed_frequency = count as f64 / self.analyzed as f64;
            let expected_frequency = expected_frequency(digit);

            DigitResult {
                digit,
                count,
                observed_frequency,
                expected_frequency,
                difference: observed_frequency - expected_frequency,
            }
        });

        let mad = digits
            .iter()
            .map(|result| result.difference.abs())
            .sum::<f64>()
            / 9.0;

        Ok(BenfordReport {
            digits,
            analyzed: self.analyzed,
            zeros: self.zeros,
            missing: self.missing,
            invalid: self.invalid,
            mad,
        })
    }
}

impl BenfordReport {
    pub fn skipped(&self) -> u64 {
        self.zeros + self.missing + self.invalid
    }

    pub fn total_received(&self) -> u64 {
        self.analyzed + self.skipped()
    }
}

pub fn expected_frequency(digit: u8) -> f64 {
    debug_assert!((1..=9).contains(&digit));

    (1.0 + 1.0 / f64::from(digit)).log10()
}
