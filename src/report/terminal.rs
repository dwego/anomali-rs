use std::path::Path;

use crate::benford::BenfordReport;

const WIDTH: usize = 76;
const MIN_RECOMMENDED_SAMPLE_SIZE: u64 = 100;

pub fn render_benford_report(
    report: &BenfordReport,
    file: &Path,
    column: &str,
) {
    print_header();
    print_source(file, column);
    print_summary(report);
    print_sample_warning(report);
    print_distribution(report);
    print_result(report);
    print_disclaimer();
}

fn print_header() {
    println!();
    println!("{}", "═".repeat(WIDTH));
    println!("{:^WIDTH$}", "ANOMALI.RS");
    println!("{:^WIDTH$}", "First-Digit Benford Analysis");
    println!("{}", "═".repeat(WIDTH));
}

fn print_source(file: &Path, column: &str) {
    println!();
    println!("  Source");
    println!("  {}", "─".repeat(WIDTH - 4));
    println!("  File:   {}", file.display());
    println!("  Column: {column}");
}

fn print_summary(report: &BenfordReport) {
    println!();
    println!("  Dataset summary");
    println!("  {}", "─".repeat(WIDTH - 4));

    println!(
        "  {:<22} {:>12}",
        "Total values",
        report.total_received()
    );

    println!(
        "  {:<22} {:>12}",
        "Analyzed",
        report.analyzed
    );

    println!(
        "  {:<22} {:>12}",
        "Zero values",
        report.zeros
    );

    println!(
        "  {:<22} {:>12}",
        "Missing values",
        report.missing
    );

    println!(
        "  {:<22} {:>12}",
        "Invalid values",
        report.invalid
    );

    println!(
        "  {:<22} {:>12}",
        "Skipped",
        report.skipped()
    );
}

fn print_sample_warning(report: &BenfordReport) {
    if report.analyzed >= MIN_RECOMMENDED_SAMPLE_SIZE {
        return;
    }

    println!();
    println!("  Warning");
    println!("  {}", "─".repeat(WIDTH - 4));
    println!(
        "  This dataset contains only {} analyzable values.",
        report.analyzed
    );
    println!(
        "  Benford analysis is more reliable with larger datasets."
    );
}

fn print_distribution(report: &BenfordReport) {
    println!();
    println!("  Digit distribution");
    println!("  {}", "─".repeat(WIDTH - 4));

    println!(
        "  {:>5}  {:>10}  {:>12}  {:>12}  {:>12}",
        "Digit",
        "Count",
        "Observed",
        "Expected",
        "Difference"
    );

    println!("  {}", "─".repeat(WIDTH - 4));

    for result in &report.digits {
        println!(
            "  {:>5}  {:>10}  {:>11.2}%  {:>11.2}%  {:>+11.2}%",
            result.digit,
            result.count,
            percentage(result.observed_frequency),
            percentage(result.expected_frequency),
            percentage(result.difference),
        );
    }
}

fn print_result(report: &BenfordReport) {
    println!();
    println!("  Analysis result");
    println!("  {}", "─".repeat(WIDTH - 4));

    println!(
        "  {:<22} {:>12.6}",
        "Mean absolute deviation",
        report.mad
    );
}

fn print_disclaimer() {
    println!();
    println!("  Note");
    println!("  {}", "─".repeat(WIDTH - 4));
    println!("  A statistical deviation does not prove fraud or manipulation.");
    println!("  The result should be used only to identify data for further review.");
    println!();
    println!("{}", "═".repeat(WIDTH));
    println!();
}

fn percentage(value: f64) -> f64 {
    value * 100.0
}
