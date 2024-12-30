use std::collections::HashSet;
use std::io::{BufRead, Read};
use std::{error::Error, fs::File, io::BufReader};
fn read(filename: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut line = line.split(",");
        let x = line.next().unwrap().parse::<usize>()?;
        let y = line.next().unwrap().parse::<usize>()?;
        output.push((x, y));
    }
    Ok(output)
}

#[derive(Debug, Eq, PartialEq)]
struct Element {
    x: usize,
    y: usize,
    cost: usize,
}
impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub mod part1 {
    use std::{
        collections::{BinaryHeap, HashMap, HashSet},
        usize,
    };

    use super::{Element, Error};
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let bs = super::read("src/day18/input.txt")?;
        let mut blocks = HashSet::new();
        for index in 0..1024 {
            blocks.insert(bs[index]);
        }

        let mut grid = BinaryHeap::new();
        let mut distance = HashMap::new();
        let max_x = 70;
        let max_y = 70;
        for y in 0..=max_x {
            for x in 0..=max_y {
                if !blocks.contains(&(x, y)) {
                    distance.insert((x, y), usize::MAX);
                }
            }
        }
        grid.push(Element {
            x: 0,
            y: 0,
            cost: 0,
        });
        let mut visited = HashSet::new();
        while let Some(el) = grid.pop() {
            if visited.contains(&(el.x, el.y)) {
                continue;
            }
            visited.insert((el.x, el.y));
            distance.insert((el.x, el.y), el.cost);
            let diffs: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for pos in diffs {
                let pos = (el.x as i64 + pos.0, el.y as i64 + pos.1);
                if pos.0 < 0 || pos.1 < 0 {
                    continue;
                }
                let pos = (pos.0 as usize, pos.1 as usize);

                if distance.contains_key(&pos) && !visited.contains(&pos) {
                    grid.push(Element {
                        x: pos.0,
                        y: pos.1,
                        cost: el.cost + 1,
                    })
                }
            }
        }

        println!("The cost is: {:?}", *distance.get(&(max_x, max_y)).unwrap());
        Ok(())
    }
}

pub mod part2 {
    use std::{
        collections::{BinaryHeap, HashMap, HashSet},
        usize,
    };

    use super::{Element, Error};
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let bs = super::read("src/day18/input.txt")?;
        for i in 0..bs.len() {
            let mut blocks = HashSet::new();
            for index in 0..=i {
                blocks.insert(bs[index]);
            }

            let mut grid = BinaryHeap::new();
            let mut distance = HashMap::new();
            let max_x = 70;
            let max_y = 70;
            for y in 0..=max_x {
                for x in 0..=max_y {
                    if !blocks.contains(&(x, y)) {
                        distance.insert((x, y), usize::MAX);
                    }
                }
            }
            grid.push(Element {
                x: 0,
                y: 0,
                cost: 0,
            });
            let mut visited = HashSet::new();
            while let Some(el) = grid.pop() {
                if visited.contains(&(el.x, el.y)) {
                    continue;
                }
                visited.insert((el.x, el.y));
                distance.insert((el.x, el.y), el.cost);
                let diffs: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for pos in diffs {
                    let pos = (el.x as i64 + pos.0, el.y as i64 + pos.1);
                    if pos.0 < 0 || pos.1 < 0 {
                        continue;
                    }
                    let pos = (pos.0 as usize, pos.1 as usize);

                    if distance.contains_key(&pos) && !visited.contains(&pos) {
                        grid.push(Element {
                            x: pos.0,
                            y: pos.1,
                            cost: el.cost + 1,
                        })
                    }
                }
            }
            if *distance.get(&(max_x, max_y)).unwrap() == usize::MAX {
                println!("The element is: {:?}", bs[i]);
                break;
            }
        }
        Ok(())
    }
}
