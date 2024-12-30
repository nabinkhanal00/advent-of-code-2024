use std::{
    collections,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
pub fn read() -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open("src/day5/input.txt")?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}
struct Comparator {
    map: collections::HashSet<(i64, i64)>,
}
impl Comparator {
    fn is_sorted(&self, page_numbers: &Vec<i64>) -> bool {
        for i in 0..(page_numbers.len() - 1) {
            for j in (i + 1)..(page_numbers.len()) {
                if !self.map.contains(&(page_numbers[i], page_numbers[j])) {
                    return false;
                }
            }
        }
        true
    }
    fn sort(&self, page_numbers: &mut Vec<i64>) -> bool {
        let mut sorted = false;
        for i in 0..(page_numbers.len() - 1) {
            for j in (i + 1)..(page_numbers.len()) {
                if !self.map.contains(&(page_numbers[i], page_numbers[j])) {
                    (page_numbers[i], page_numbers[j]) = (page_numbers[j], page_numbers[i]);
                    sorted = true;
                }
            }
        }
        sorted
    }
}
pub mod part1 {
    use std::{collections, error::Error, io::BufRead};

    use super::{read, Comparator};

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = read()?;
        let mut ordering = true;
        let mut comparator = Comparator {
            map: collections::HashSet::new(),
        };
        let mut sum: i64 = 0;
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                ordering = false;
                continue;
            }
            if ordering {
                let mut order = line.split("|");
                let first = order.next().unwrap().parse::<i64>()?;
                let second = order.next().unwrap().parse::<i64>()?;
                comparator.map.insert((first, second));
            } else {
                let page_numbers: Vec<i64> = line
                    .split(",")
                    .map(|val| val.parse::<i64>().unwrap())
                    .collect();
                if comparator.is_sorted(&page_numbers) {
                    sum += page_numbers[page_numbers.len() / 2];
                }
            }
        }
        println!("The sum is: {sum}");
        Ok(())
    }
}

pub mod part2 {
    use super::{read, Comparator};
    use std::collections;
    use std::error::Error;
    use std::io::BufRead;
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = read()?;
        let mut ordering = true;
        let mut comparator = Comparator {
            map: collections::HashSet::new(),
        };
        let mut sum: i64 = 0;
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                ordering = false;
                continue;
            }
            if ordering {
                let mut order = line.split("|");
                let first = order.next().unwrap().parse::<i64>()?;
                let second = order.next().unwrap().parse::<i64>()?;
                comparator.map.insert((first, second));
            } else {
                let mut page_numbers: Vec<i64> = line
                    .split(",")
                    .map(|val| val.parse::<i64>().unwrap())
                    .collect();
                if comparator.sort(&mut page_numbers) {
                    sum += page_numbers[page_numbers.len() / 2];
                }
            }
        }
        println!("The sum is: {sum}");
        Ok(())
    }
}
