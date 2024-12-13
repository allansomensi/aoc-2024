use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::part_one::{calculate_report_status, Status};

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

                if calculate_report_status(report.clone()) == Status::Safe {
                    safe_reports += 1;
                } else {
                    // Try removing some level from the report that makes it Safe
                    for (i, _) in report.iter().enumerate() {
                        let mut temp_report = report.clone();
                        temp_report.remove(i);

                        if calculate_report_status(temp_report) == Status::Safe {
                            safe_reports += 1;
                            break; // Stop if a Safe is returned
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {e}"),
        }
    }

    println!("Safe Reports: {safe_reports}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part_one::{calculate_report_status, Status};

    fn calculates_with_discount(report: Vec<u8>) -> u32 {
        let mut safe_reports = 0;

        if calculate_report_status(report.clone()) == Status::Safe {
            safe_reports += 1;
        } else {
            // Try removing some level from the report that makes it Safe
            for (i, _) in report.iter().enumerate() {
                let mut temp_report = report.clone();
                temp_report.remove(i);

                if calculate_report_status(temp_report) == Status::Safe {
                    safe_reports += 1;
                    break; // Stop if a Safe is returned
                }
            }
        }
        safe_reports
    }

    #[test]
    fn test_safe_incresing_report() {
        let safe_report = vec![1, 3, 2, 4, 5];
        assert_eq!(calculates_with_discount(safe_report), 1);
    }

    #[test]
    fn test_safe_decreasing_report() {
        let safe_report = vec![8, 6, 4, 4, 1];
        assert_eq!(calculates_with_discount(safe_report), 1);
    }

    #[test]
    fn test_unsafe_incresing_report() {
        let unsafe_report = vec![1, 2, 7, 8, 9];
        assert_eq!(calculates_with_discount(unsafe_report), 0);
    }

    #[test]
    fn test_unsafe_decreasing_report() {
        let unsafe_report = vec![9, 7, 6, 2, 1];
        assert_eq!(calculates_with_discount(unsafe_report), 0);
    }
}
