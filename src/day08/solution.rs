use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
pub fn read() -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open("src/day8/input.txt")?;
    let reader = io::BufReader::new(file);

    Ok(reader)
}
pub mod part1 {
    use std::{collections::HashMap, collections::HashSet, error::Error, io::BufRead};

    use super::read;
    fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
        return (a.0 - b.0).abs() + (a.1 - b.1).abs();
    }
    fn satisfies(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        let collinear = a.1 * (b.0 - c.0) + b.1 * (c.0 - a.0) + c.1 * (a.0 - b.0) == 0;
        collinear && (distance(a, b) == 2 * distance(a, c) || 2 * distance(a, b) == distance(a, c))
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = read()?;
        let mut ant_positions: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
        let mut size: usize = 0;
        for (y, line) in reader.lines().enumerate() {
            size += 1;
            for (x, antenna) in line?.chars().enumerate() {
                if !antenna.is_alphanumeric() {
                    continue;
                }
                if let Some(positions) = ant_positions.get_mut(&antenna) {
                    positions.push((y as i64, x as i64));
                } else {
                    ant_positions.insert(antenna, vec![(y as i64, x as i64)]);
                }
            }
        }

        let mut combinations: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
        for (_, positions) in ant_positions {
            let pairs: Vec<((i64, i64), (i64, i64))> = positions
                .iter()
                .enumerate()
                .flat_map(|(i, &pos1)| positions[i + 1..].iter().map(move |&pos2| (pos1, pos2)))
                .collect();
            for pair in pairs {
                combinations.insert(pair);
            }
        }
        let mut result = HashSet::new();
        for y in 0..size {
            for x in 0..size {
                if result.contains(&(y as i64, x as i64)) {
                    continue;
                }
                for (first, second) in &combinations {
                    let point = (y as i64, x as i64);
                    if satisfies(point, *first, *second) {
                        result.insert(point);
                    }
                }
            }
        }
        println!("The count is: {}.", result.len());
        Ok(())
    }
}

pub mod part2 {
    use std::{collections::HashMap, collections::HashSet, error::Error, io::BufRead};

    use super::read;
    fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
        return (a.0 - b.0).abs() + (a.1 - b.1).abs();
    }
    fn satisfies(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        let collinear = a.1 * (b.0 - c.0) + b.1 * (c.0 - a.0) + c.1 * (a.0 - b.0) == 0;
        collinear
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = read()?;
        let mut ant_positions: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
        let mut size: usize = 0;
        for (y, line) in reader.lines().enumerate() {
            size += 1;
            for (x, antenna) in line?.chars().enumerate() {
                if !antenna.is_alphanumeric() {
                    continue;
                }
                if let Some(positions) = ant_positions.get_mut(&antenna) {
                    positions.push((y as i64, x as i64));
                } else {
                    ant_positions.insert(antenna, vec![(y as i64, x as i64)]);
                }
            }
        }

        let mut combinations: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
        for (_, positions) in ant_positions {
            let pairs: Vec<((i64, i64), (i64, i64))> = positions
                .iter()
                .enumerate()
                .flat_map(|(i, &pos1)| positions[i + 1..].iter().map(move |&pos2| (pos1, pos2)))
                .collect();
            for pair in pairs {
                combinations.insert(pair);
            }
        }
        let mut result = HashSet::new();
        for y in 0..size {
            for x in 0..size {
                if result.contains(&(y as i64, x as i64)) {
                    continue;
                }
                for (first, second) in &combinations {
                    let point = (y as i64, x as i64);
                    if satisfies(point, *first, *second) {
                        result.insert(point);
                    }
                }
            }
        }
        println!("The count is: {}.", result.len());
        Ok(())
    }
}
