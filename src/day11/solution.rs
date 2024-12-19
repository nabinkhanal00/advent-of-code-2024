use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<Vec<i64>, Box<dyn Error>> {
    let file = File::open("src/day11/input.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let numbers: Vec<i64> = line
        .split(" ")
        .map(|num| num.trim().parse().unwrap())
        .collect();
    Ok(numbers)
}

pub mod part1 {
    use super::read;
    use std::error::Error;
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let mut numbers = read()?;
        for _ in 0..25 {
            let mut new_numbers = Vec::new();
            for number in &numbers {
                if *number == 0 {
                    new_numbers.push(1);
                    continue;
                }
                let dig = ((*number as f64).log10().floor() as i64) + 1;
                if dig % 2 == 0 {
                    let factor = (10 as i64).pow((dig / 2) as u32);
                    new_numbers.push(number / factor);
                    new_numbers.push(number % factor);
                } else {
                    new_numbers.push(number * 2024);
                }
            }
            numbers = new_numbers;
        }
        println!("The count is: {}.", numbers.len());
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::{collections::HashMap, error::Error};
    fn find(num: i64, c: i64, m: &mut HashMap<(i64, i64), i64>) -> i64 {
        if let Some(count) = m.get(&(num, c)) {
            return *count;
        }
        if c == 0 {
            return 1;
        }
        let count;
        if num == 0 {
            count = find(1, c - 1, m);
        } else {
            let dig = ((num as f64).log10().floor() as i64) + 1;
            if dig % 2 == 0 {
                let factor = (10 as i64).pow((dig / 2) as u32);
                let first = num / factor;
                let second = num % factor;
                count = find(first, c - 1, m) + find(second, c - 1, m);
            } else {
                count = find(num * 2024, c - 1, m);
            }
        }
        m.insert((num, c), count);
        count
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let numbers = read()?;
        let mut res = HashMap::new();
        let mut count = 0;
        for number in &numbers {
            count += find(*number, 75, &mut res);
        }
        println!("The count is: {}.", count);
        Ok(())
    }
}
