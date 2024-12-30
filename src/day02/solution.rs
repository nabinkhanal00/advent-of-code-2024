use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
fn read() -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let file = File::open("src/day02/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut reports = Vec::new();
    for line in reader.lines() {
        let mut report = Vec::new();
        let line = line?;
        let mut values = line.split(" ");
        while let Some(value) = values.next() {
            let value: i64 = value.parse()?;
            report.push(value);
        }
        reports.push(report);
    }
    Ok(reports)
}
pub mod part1 {
    use std::error::Error;

    use super::read;
    fn check_increasing(report: Vec<i64>) -> bool {
        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff > 0 && diff < 4 {
                continue;
            } else {
                return false;
            }
        }
        true
    }
    fn check_decreasing(report: Vec<i64>) -> bool {
        for i in 1..report.len() {
            let diff = report[i - 1] - report[i];
            if diff > 0 && diff < 4 {
                continue;
            } else {
                return false;
            }
        }
        true
    }

    fn is_report_safe(report: Vec<i64>) -> bool {
        let first = report[0];
        let second = report[1];
        if first < second {
            check_increasing(report)
        } else if first > second {
            check_decreasing(report)
        } else {
            false
        }
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reports = read()?;
        let mut count = 0;
        for report in reports {
            if is_report_safe(report) {
                count += 1;
            }
        }
        println!("Safe reports: {count}");
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::error::Error;

    fn is_report_safe(report: Vec<i64>) -> bool {
        let mut corrected = false;
        let mut prev = report[1] - report[0];
        for i in 2..report.len() {
            let mut difference = report[i] - report[i - 1];
            if prev * difference <= 0 {
                if corrected {
                    return false;
                }
                prev = difference + prev;
                difference = prev;
                corrected = true;
            }
            if prev.abs() >= 4 {
                if i == 2 {
                    corrected = true;
                } else {
                    return false;
                }
            }
            prev = difference;
        }
        if prev.abs() >= 4 && corrected {
            return false;
        }
        true
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        let reports = read()?;
        for report in reports {
            if is_report_safe(report) {
                count += 1;
            }
        }
        println!("Safe reports: {count}");
        Ok(())
    }
}
