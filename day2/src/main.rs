#![feature(array_windows)]

use std::{ops::Not, str::FromStr};

const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

type Level = i64;
type Report = Vec<Level>;

fn parse_report(line: &str) -> Report {
    line.split_whitespace()
        .map(|chars| Level::from_str(chars).unwrap())
        .collect()
}

fn is_report_safe(report: &Report) -> bool {
    // All increasing OR all decreasing (no consecutive numbers can be the same)
    // Gap between 1 and 3 (inclusive)
    let gaps: Vec<_> = report.array_windows().map(|[a, b]| b - a).collect();
    let increasing = gaps.iter().all(|gap| gap > &0);
    let decreasing = gaps.iter().all(|gap| gap < &0);
    let size_ok = gaps.iter().all(|gap| (1..=3).contains(&gap.abs()));
    (increasing || decreasing) && size_ok
}

fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(parse_report).collect()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reports = parse_input(&input);
    for report in &reports {
        assert!(report.is_empty().not())
    }
    let q1 = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();
    println!("Q1: {q1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let reports = parse_input(TEST_INPUT);
        for report in &reports {
            assert!(report.is_empty().not())
        }
        let q1 = reports
            .iter()
            .filter(|report| is_report_safe(report))
            .count();
        assert_eq!(q1, 2);
    }
}
