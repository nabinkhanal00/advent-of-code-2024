use std::{
    fs::File,
    io::{self, BufRead},
};
fn read() -> Result<(Vec<u64>, Vec<u64>), std::io::Error> {
    let file = File::open("src/day1/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut firsts = Vec::new();
    let mut seconds = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut values = line.split("   ");
        let first = values.next().unwrap().trim();
        let first: u64 = first.parse().unwrap();
        let second = values.next().unwrap().trim();
        let second: u64 = second.parse().unwrap();
        firsts.push(first);
        seconds.push(second);
    }
    Ok((firsts, seconds))
}
pub mod part1 {
    use crate::day1::solution::read;

    pub fn solve() -> Result<(), std::io::Error> {
        let mut sum: u64 = 0;
        let (mut firsts, mut seconds) = read()?;
        firsts.sort();
        seconds.sort();
        for i in 0..firsts.len() {
            sum += firsts[i].abs_diff(seconds[i]);
        }
        println!("Sum is: {}", sum);
        Ok(())
    }
}

pub mod part2 {
    use crate::day1::solution::read;
    pub fn solve() -> Result<(), std::io::Error> {
        let (mut firsts, mut seconds) = read()?;

        firsts.sort();
        seconds.sort();
        let mut similarity: u64 = 0;
        let mut start: usize = 0;
        for num in firsts {
            let mut count = 0;
            for i in start..seconds.len() {
                if num > seconds[i] {
                    start = i;
                    continue;
                } else if num == seconds[i] {
                    count += 1;
                } else {
                    break;
                }
            }
            similarity += count * num;
        }
        println!("Similarity is: {similarity}");
        Ok(())
    }
}
