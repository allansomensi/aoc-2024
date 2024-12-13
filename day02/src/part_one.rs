use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Represents the status possibilities for a report
#[derive(Debug, PartialEq)]
pub enum Status {
    Safe,
    Unsafe,
}

pub fn run() -> std::io::Result<()> {
    // Open the puzzle input with the lists of reports
    let reports_file = File::open("src/input.txt")?;
    let reader = BufReader::new(reports_file);

    let mut safe_reports: u32 = 0;

    // Reads line by line and counts how many reports are safe
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let report: Vec<u8> = line
                    .split_whitespace()
                    .map(|num| num.parse().expect("Error parsing data"))
                    .collect();

                if calculate_report_status(report) == Status::Safe {
                    safe_reports += 1
                }
            }
            Err(e) => eprintln!("Error reading line: {e}"),
        }
    }

    println!("Safe Reports: {safe_reports}");

    Ok(())
}

pub fn calculate_report_status(report: Vec<u8>) -> Status {
    // The report must have at least two levels
    if report.len() < 2 {
        return Status::Unsafe;
    }

    let mut status: Status = Status::Unsafe;
    let mut last = report[0];

    // Checks whether the report is increasing or decreasing
    let is_incresing = match report[1] {
        r if r > last => true,
        r if r < last => false,
        _ => return Status::Unsafe, // If the first two elements are equal, return Unsafe
    };

    // Skip the first level and calculate the status by iterating through each item in the report
    for level in report.iter().skip(1) {
        // If the levels are equal, return Unsafe
        if level == &last {
            return Status::Unsafe;
        }

        // Checks if the sequence is inconsistent
        if is_incresing && level < &last || !is_incresing && level > &last {
            return Status::Unsafe;
        }

        // Check if the absolute difference is in the range [1, 3]
        if level.abs_diff(last) >= 1 && level.abs_diff(last) <= 3 {
            status = Status::Safe;
        } else {
            return Status::Unsafe;
        }

        // Saves the last iterated level, to compare whether the report is increasing or decreasing
        last = *level;
    }
    status
}

#[cfg(test)]
mod tests {
    use crate::part_one::Status;

    use super::calculate_report_status;

    #[test]
    fn test_safe_incresing_report() {
        let safe_report = vec![1, 3, 6, 7, 9];
        assert_eq!(calculate_report_status(safe_report), Status::Safe);
    }

    #[test]
    fn test_safe_decreasing_report() {
        let safe_report = vec![7, 6, 4, 2, 1];
        assert_eq!(calculate_report_status(safe_report), Status::Safe);
    }

    #[test]
    fn test_unsafe_incresing_report() {
        let unsafe_report = vec![1, 3, 2, 4, 5];
        assert_eq!(calculate_report_status(unsafe_report), Status::Unsafe);
    }

    #[test]
    fn test_unsafe_decreasing_report() {
        let unsafe_report = vec![9, 7, 6, 2, 1];
        assert_eq!(calculate_report_status(unsafe_report), Status::Unsafe);
    }

    #[test]
    fn test_inconstant_report() {
        let unsafe_report = vec![4, 3, 2, 2, 1];
        assert_eq!(calculate_report_status(unsafe_report), Status::Unsafe);
    }
}
