use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<Vec<(i64, i64, i64, i64)>, Box<dyn Error>> {
    let file = File::open("src/day14/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::new();
    let re = Regex::new(r"[+-]?\d+").unwrap();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i64> = re
            .find_iter(line.as_str())
            .filter_map(|n| n.as_str().parse::<i64>().ok())
            .collect();
        result.push((numbers[0], numbers[1], numbers[2], numbers[3]));
    }
    Ok(result)
}

pub mod part1 {
    use super::read;
    use std::error::Error;

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let mut robots = read()?;
        let w = 101;
        let h = 103;
        for robot in &mut robots {
            for _ in 0..100 {
                robot.0 += robot.2 + w;
                robot.1 += robot.3 + h;
                robot.0 %= w;
                robot.1 %= h;
            }
        }
        let mut count: Vec<i64> = vec![0, 0, 0, 0];
        let mw = w / 2;
        let mh = h / 2;
        for robot in robots {
            if robot.0 < mw && robot.1 < mh {
                count[0] += 1;
            } else if robot.0 > mw && robot.1 < mh {
                count[1] += 1;
            } else if robot.0 < mw && robot.1 > mh {
                count[2] += 1;
            } else if robot.0 > mw && robot.1 > mh {
                count[3] += 1;
            }
        }
        println!(
            "The result is: {}.",
            count[0] * count[1] * count[2] * count[3]
        );
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::{collections::HashSet, error::Error};

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let mut robots = read()?;
        let w = 101;
        let h = 103;
        let mut iterations = 0;
        let mut prev = 0 as i64;
        let mut iter = 0;
        loop {
            let mut hs = HashSet::new();
            for robot in &mut robots {
                robot.0 += robot.2 + w;
                robot.1 += robot.3 + h;
                robot.0 %= w;
                robot.1 %= h;
                hs.insert((robot.0, robot.1));
            }
            iterations += 1;
            if hs.len() as i64 > prev {
                iter = iterations;
                prev = hs.len() as i64;
            }
            // if iterations == 6870 {
            //     let mut grid = vec![vec!["🟥"; 101]; 103];
            //     for robot in robots {
            //         grid[robot.1 as usize][robot.0 as usize] = "⬜️";
            //     }
            //     for row in grid {
            //         println!("{}", row.into_iter().collect::<String>());
            //     }
            //     break;
            // }
            if iterations > 100000 {
                break;
            }
        }
        println!("The result is: {}.", iter);
        Ok(())
    }
}
