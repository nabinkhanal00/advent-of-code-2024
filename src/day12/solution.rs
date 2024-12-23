use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};
pub fn read() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("src/day12/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?.chars().collect();
        result.push(line);
    }
    Ok(result)
}

pub mod part1 {
    use super::read;
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        error::Error,
    };
    fn get_surroundings((y, x): (i64, i64)) -> Vec<(i64, i64)> {
        vec![(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)]
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let grid = read()?;
        let mut pos_map = HashMap::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                let m = pos_map.entry(element).or_insert(HashSet::new());
                m.insert((y as i64, x as i64));
            }
        }
        let mut reg_map = HashMap::new();
        for (&plant_type, positions) in &mut pos_map {
            let mut regions: Vec<HashSet<(i64, i64)>> = Vec::new();
            while positions.len() > 0 {
                let pos = positions.iter().next().unwrap().clone();
                let mut queue = VecDeque::new();
                queue.push_back(pos);

                let mut region = HashSet::new();
                while queue.len() > 0 {
                    let pos = queue.pop_front().unwrap();
                    region.insert(pos);
                    positions.remove(&pos);
                    for sur_pos in get_surroundings(pos) {
                        if positions.contains(&sur_pos) && !region.contains(&sur_pos) {
                            queue.push_back(sur_pos);
                        }
                    }
                }
                regions.push(region);
            }
            reg_map.insert(plant_type, regions);
        }

        let mut sum = 0 as i64;
        for (_, regions) in reg_map {
            for region in &regions {
                let area = region.len() as i64;
                let mut perimeter = 0 as i64;
                for pos in region {
                    for sur_pos in get_surroundings(*pos) {
                        if !region.contains(&sur_pos) {
                            perimeter += 1;
                        }
                    }
                }
                sum += area * perimeter;
            }
        }

        println!("The price is: {}", sum);
        Ok(())
    }
}

pub mod part2 {
    use super::read;
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        error::Error,
        io::{self, Write},
    };
    fn get_surroundings() -> Vec<(i64, i64)> {
        vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let grid = read()?;
        let mut pos_map = HashMap::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                let m = pos_map.entry(element).or_insert(HashSet::new());
                m.insert((y as i64, x as i64));
            }
        }
        let mut reg_map = HashMap::new();
        let mut count = 0;
        for (&plant_type, positions) in &mut pos_map {
            let mut regions: Vec<HashSet<(i64, i64)>> = Vec::new();
            while positions.len() > 0 {
                let pos = positions.iter().next().unwrap().clone();
                let mut queue = VecDeque::new();
                queue.push_back(pos);

                let mut region = HashSet::new();
                while queue.len() > 0 {
                    let pos = queue.pop_front().unwrap();
                    region.insert(pos);
                    positions.remove(&pos);
                    for diff in get_surroundings() {
                        let sur_pos = (pos.0 + diff.0, pos.1 + diff.1);
                        if positions.contains(&sur_pos) && !region.contains(&sur_pos) {
                            queue.push_back(sur_pos);
                        }
                    }
                }
                regions.push(region);
                count += 1;
            }
            reg_map.insert(plant_type, regions);
        }

        let mut sum = 0 as i64;
        let _ = io::stdout().flush();
        for (_, regions) in reg_map {
            for region in &regions {
                let area = region.len() as i64;
                let mut sides = 0 as i64;
                for pos in region {
                    for diff in get_surroundings() {
                        let sur_pos = (pos.0 + diff.0, pos.1 + diff.1);
                        let complement_a = (diff.1 * -1, diff.0);
                        // let complement_c = (diff.1, diff.0 * -1);
                        if !region.contains(&sur_pos)
                            && (!region
                                .contains(&((pos.0 + complement_a.0), (pos.1 + complement_a.1)))
                                || region.contains(&(
                                    (pos.0 + complement_a.0 + diff.0),
                                    (pos.1 + complement_a.1 + diff.1),
                                )))
                        {
                            sides += 1;
                        }
                    }
                }
                sum += area * sides;
            }
        }

        println!("The price is: {}", sum);
        Ok(())
    }
}
