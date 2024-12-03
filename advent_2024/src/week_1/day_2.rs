use std::fs::File;
use std::io::{self, Read};

pub fn main() -> io::Result<()> {
    const PATH: &str = "src/inputs/in_2";

    fn is_safe_report(report: &[i32]) -> bool {
        if is_valid_sequence(report) {
            return true;
        }

        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i);
            if is_valid_sequence(&modified_report) {
                return true;
            }
        }

        false
    }
    fn is_valid_sequence(report: &[i32]) -> bool {
        if report.len() < 2 {
            return true;
        }

        let mut inc: bool = true;
        let mut dec: bool = true;

        for win in report.windows(2) {
            let diff = (win[0] - win[1]).abs();

            if diff < 1 || diff > 3 {
                return false;
            }
            if win[1] > win[0] {
                dec = false
            } else if win[1] < win[0] {
                inc = false
            }
        }
        inc || dec
    }

    let mut file = File::open(PATH)?;
    let mut input = String::new();
    let mut safe_reports_nrml = 0;
    let mut safe_reports_extra = 0;
    file.read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let report_numbers: Result<Vec<i32>, _> =
            line.split_whitespace().map(|x| x.parse::<i32>()).collect();

        match report_numbers {
            Ok(report) => {
                if is_safe_report(&report) {
                    safe_reports_extra += 1
                }
                if is_valid_sequence(&report) {
                    safe_reports_nrml += 1
                }
            }
            Err(e) => {
                eprintln!("Failed to parse line '{}': {}", line, e);
            }
        }
    }
    println!("The number of safe reports is {}", safe_reports_nrml);
    println!("The number of safe reports is {}", safe_reports_extra);
    Ok(())
}
