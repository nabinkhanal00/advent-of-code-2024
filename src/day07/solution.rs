use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
pub fn read() -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open("src/day7/input.txt")?;
    let reader = io::BufReader::new(file);

    Ok(reader)
}
pub mod part1 {
    use std::{collections::HashSet, error::Error, io::BufRead};

    use super::read;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = read()?;
        let mut sum: i64 = 0;
        for line in reader.lines() {
            let line = line?;
            let mut splits = line.split(":");
            let res: i64 = splits.next().unwrap().parse()?;
            let mut numbers = splits.next().unwrap().trim().split(" ");
            let mut results = HashSet::new();
            while let Some(number) = numbers.next() {
                let number: i64 = number.parse()?;
                if results.len() == 0 {
                    results.insert(number);
                } else {
                    let mut newresults = HashSet::new();
                    for element in &results {
                        newresults.insert(element + number);
                        newresults.insert(element * number);
                    }
                    results = newresults;
                }
            }
            if results.contains(&res) {
                sum += res;
            }
        }
        println!("The sum is: {sum}");
        Ok(())
    }
}

pub mod part2 {
    use std::{collections::HashSet, error::Error, io::BufRead};

    use super::read;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = read()?;
        let mut sum: i64 = 0;
        for line in reader.lines() {
            let line = line?;
            let mut splits = line.split(":");
            let res: i64 = splits.next().unwrap().parse()?;
            let mut numbers = splits.next().unwrap().trim().split(" ");
            let mut results = HashSet::new();
            while let Some(number) = numbers.next() {
                let number: i64 = number.parse()?;
                if results.len() == 0 {
                    results.insert(number);
                } else {
                    let mut newresults = HashSet::new();
                    for element in &results {
                        newresults.insert(element + number);
                        newresults.insert(element * number);
                        let no_of_digits = f64::log10(number as f64) as i64 + 1;
                        newresults.insert(element * (10 as i64).pow(no_of_digits as u32) + number);
                    }
                    results = newresults;
                }
            }
            if results.contains(&res) {
                sum += res;
            }
        }
        println!("The sum is: {sum}");
        Ok(())
    }
}
