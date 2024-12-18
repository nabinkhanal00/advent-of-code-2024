use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
pub fn read() -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let file = File::open("src/day10/input.txt")?;
    let reader = io::BufReader::new(file);
    let trail: Vec<Vec<i64>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap_or_else(|| 100) as i64)
                .collect()
        })
        .collect();
    Ok(trail)
}

pub mod part1 {
    use super::read;
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };
    fn trail_heads(trail: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        let length = trail.len();
        for y in 0..length {
            for x in 0..length {
                if trail[y][x] == 0 {
                    trailheads.push((y, x));
                }
            }
        }
        trailheads
    }
    pub fn count_paths() -> Result<(), Box<dyn Error>> {
        let trail = read()?;
        let mut m = HashMap::new();
        let trailheads = trail_heads(&trail);
        let length = trail.len();
        for y in 0..length {
            for x in 0..length {
                let e = m.entry(trail[y][x]).or_insert(HashSet::new());
                e.insert((y as i64, x as i64));
            }
        }
        let mut dp = vec![vec![0 as i64; length]; length];
        for key in (0..=9).rev() {
            let value = m.get(&key).unwrap();
            if key == 9 {
                for v in value {
                    dp[v.0 as usize][v.1 as usize] = 1;
                }
            } else {
                let previous = m.get(&(key + 1)).unwrap();
                for v in value {
                    let mut count: i64 = 0;
                    let y = v.0;
                    let x = v.1;
                    if previous.contains(&(y, x - 1)) {
                        count += dp[(y) as usize][(x - 1) as usize];
                    }
                    if previous.contains(&(y, x + 1)) {
                        count += dp[(y) as usize][(x + 1) as usize];
                    }
                    if previous.contains(&(y + 1, x)) {
                        count += dp[(y + 1) as usize][(x) as usize];
                    }
                    if previous.contains(&(y - 1, x)) {
                        count += dp[(y - 1) as usize][(x) as usize];
                    }
                    dp[v.0 as usize][v.1 as usize] = count;
                }
            }
        }
        let mut sum = 0;
        for p in &dp {
            println!("{:?}", p);
        }
        for head in trailheads {
            println!("{}", dp[head.0][head.1]);
            sum += dp[head.0][head.1];
        }

        Ok(())
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let trail = read()?;
        let mut m = HashMap::new();
        let trailheads = trail_heads(&trail);
        let length = trail.len();
        for y in 0..length {
            for x in 0..length {
                let e = m.entry(trail[y][x]).or_insert(HashSet::new());
                e.insert((y as i64, x as i64));
            }
        }
        let mut dp = vec![vec![HashSet::new(); length]; length];
        for key in (0..=9).rev() {
            let value = m.get(&key).unwrap();
            if key == 9 {
                for v in value {
                    dp[v.0 as usize][v.1 as usize].insert((v.0, v.1));
                }
            } else {
                let previous = m.get(&(key + 1)).unwrap();
                for v in value {
                    let y = v.0;
                    let x = v.1;
                    if previous.contains(&(y, x - 1)) {
                        let h = dp[(y) as usize][(x - 1) as usize].clone();
                        for elem in h {
                            dp[v.0 as usize][v.1 as usize].insert(elem);
                        }
                    }
                    if previous.contains(&(y, x + 1)) {
                        let h = dp[(y) as usize][(x + 1) as usize].clone();
                        for elem in h {
                            dp[v.0 as usize][v.1 as usize].insert(elem);
                        }
                    }
                    if previous.contains(&(y + 1, x)) {
                        let h = dp[(y + 1) as usize][(x) as usize].clone();
                        for elem in h {
                            dp[v.0 as usize][v.1 as usize].insert(elem);
                        }
                    }
                    if previous.contains(&(y - 1, x)) {
                        let h = dp[(y - 1) as usize][(x) as usize].clone();
                        for elem in h {
                            dp[v.0 as usize][v.1 as usize].insert(elem);
                        }
                    }
                }
            }
        }
        let mut sum = 0;
        for head in trailheads {
            sum += dp[head.0][head.1].len();
        }
        println!("The sum is: {sum}");

        Ok(())
    }
}
pub mod part2 {

    use super::read;
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };
    fn trail_heads(trail: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        let length = trail.len();
        for y in 0..length {
            for x in 0..length {
                if trail[y][x] == 0 {
                    trailheads.push((y, x));
                }
            }
        }
        trailheads
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let trail = read()?;
        let mut m = HashMap::new();
        let trailheads = trail_heads(&trail);
        let length = trail.len();
        for y in 0..length {
            for x in 0..length {
                let e = m.entry(trail[y][x]).or_insert(HashSet::new());
                e.insert((y as i64, x as i64));
            }
        }
        let mut dp = vec![vec![0 as i64; length]; length];
        for key in (0..=9).rev() {
            let value = m.get(&key).unwrap();
            if key == 9 {
                for v in value {
                    dp[v.0 as usize][v.1 as usize] = 1;
                }
            } else {
                let previous = m.get(&(key + 1)).unwrap();
                for v in value {
                    let mut count: i64 = 0;
                    let y = v.0;
                    let x = v.1;
                    if previous.contains(&(y, x - 1)) {
                        count += dp[(y) as usize][(x - 1) as usize];
                    }
                    if previous.contains(&(y, x + 1)) {
                        count += dp[(y) as usize][(x + 1) as usize];
                    }
                    if previous.contains(&(y + 1, x)) {
                        count += dp[(y + 1) as usize][(x) as usize];
                    }
                    if previous.contains(&(y - 1, x)) {
                        count += dp[(y - 1) as usize][(x) as usize];
                    }
                    dp[v.0 as usize][v.1 as usize] = count;
                }
            }
        }
        let mut sum = 0;
        for head in trailheads {
            sum += dp[head.0][head.1];
        }

        println!("The sum is {sum}");

        Ok(())
    }
}
