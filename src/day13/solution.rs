use itertools::Itertools;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<Vec<((i64, i64, i64), (i64, i64, i64))>, Box<dyn Error>> {
    let file = File::open("src/day13/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            continue;
        }
        result.push(line);
    }
    let triplets = result.into_iter().chunks(3);
    let mut result = Vec::new();
    for chunk in &triplets {
        let chunk: Vec<String> = chunk.collect();
        let (first, second, res) = (&chunk[0], &chunk[1], &chunk[2]);
        let re = Regex::new(r"[+-]?\d+").unwrap();
        let first: Vec<i64> = re
            .find_iter(first.as_str())
            .filter_map(|n| n.as_str().parse::<i64>().ok())
            .collect();
        let second: Vec<i64> = re
            .find_iter(second.as_str())
            .filter_map(|n| n.as_str().parse::<i64>().ok())
            .collect();
        let res: Vec<i64> = re
            .find_iter(res.as_str())
            .filter_map(|n| n.as_str().parse::<i64>().ok())
            .collect();
        let mut zipped = first
            .into_iter()
            .zip(second.into_iter())
            .zip(res.into_iter())
            .map(|((x, y), z)| (x, y, z));
        result.push((zipped.next().unwrap(), zipped.next().unwrap()));
    }
    Ok(result)
}

pub mod part1 {
    use super::read;
    use std::error::Error;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let res = read()?;
        let mut sum: i64 = 0;
        for (eqa, eqb) in res {
            let mut r = None;
            for a in 0..=100 {
                for b in 0..=100 {
                    if eqa.0 * a + eqa.1 * b == eqa.2 && eqb.0 * a + eqb.1 * b == eqb.2 {
                        if let Some((pa, pb)) = r {
                            if pa * 3 + pb < a * 3 + b {
                                r = Some((pa, pb));
                            }
                        } else {
                            r = Some((a, b));
                        }
                    }
                }
            }
            if let Some((a, b)) = r {
                sum += a * 3 + b;
            }
        }

        println!("The total tokens is: {sum}");
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::error::Error;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let res = read()?;
        let mut sum: i64 = 0;
        for ((a1, b1, c1), (a2, b2, c2)) in res {
            let c1 = c1 + 10000000000000;
            let c2 = c2 + 10000000000000;

            let delta = a1 * b2 - a2 * b1;
            if delta == 0 {
                continue;
            } else {
                let delta_x = c1 * b2 - c2 * b1;
                let delta_y = a1 * c2 - a2 * c1;
                if delta_x % delta != 0 || delta_y % delta != 0 {
                    continue;
                }
                let a = delta_x / delta;
                let b = delta_y / delta;
                sum += 3 * a + b;
            }
        }

        println!("The total tokens is: {sum}");
        Ok(())
    }
}
